// Tokio includes a set of libraries for network/web service development; as well as, std async version

use bytes::Bytes;
use mini_redis::{Command, Connection, Frame};
use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;
use std::result;
use std::sync::{Arc, Mutex, MutexGuard};
use tokio::net::{TcpListener, TcpStream};
use futures::future::join_all;

type Result<T> = result::Result<T, Box<dyn Error>>;

type DATABASE = Arc<Mutex<HashMap<String, Bytes>>>;

const NUM_CONNECTIONS: usize = 10; 

mod impl_1 {
    use std::time::Duration;
    use tokio::time::sleep;
    use super::*;

    pub async fn server(num_connections: usize) -> Result<()> {
        let address = "127.0.0.1:8080";
        let listener = TcpListener::bind(address).await?;
        println!("• listening {}", address);

        let db = Arc::new(Mutex::new(HashMap::new()));

        for _ in 0..num_connections {
            let (stream, _) = listener.accept().await?;
            println!("accepted TCP connection");

            let db = db.clone();

            tokio::spawn(async move {
                let mut connection = Connection::new(stream);

                while let Some(frame) = connection.read_frame().await.unwrap() {
                    let r = match Command::from_frame(frame).unwrap() {
                        Command::Set(cmd) => {
                            let k = cmd.key().to_string();
                            let v = cmd.value().clone();

                            println!(
                                "set: key {} value {}",
                                k,
                                v.to_vec().iter().map(|i| i.to_string()).collect::<String>()
                            );

                            let mut db = db.lock().unwrap();
                            db.insert(k, v);
                            Frame::Simple("OK".to_string())
                        }
                        Command::Get(cmd) => {
                            let k = cmd.key().to_string();

                            println!("get: key {}", k);

                            let db = db.lock().unwrap();
                            if let Some(v) = db.get(&k) {
                                Frame::Bulk(v.clone())
                            } else {
                                Frame::Null
                            }
                        }
                        cmd => unimplemented!("{:?}", cmd),
                    };

                    connection.write_frame(&r).await.unwrap();
                }
            });
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        Ok(())
    }

    pub async fn client() -> Result<()> {
        let address = "127.0.0.1:8080";
        let mut client = mini_redis::client::connect(address).await.unwrap();

        client.set("x".into(), "12345".into()).await.unwrap();

        let result = client.get("x").await.unwrap().unwrap();
        println!(
            "\t value {}",
            result
                .to_vec()
                .iter()
                .map(|i| i.to_string())
                .collect::<String>()
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tokio() {
        const NUM_CONNECTIONS: usize = 10; 
    
        let server_handle = tokio::spawn(async {
            impl_1::server(NUM_CONNECTIONS).await.unwrap();
        });
    
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
        let mut client_handles = vec![];
    
        for _ in 0..NUM_CONNECTIONS {
            let handle = tokio::spawn(async {
                impl_1::client().await.unwrap();
            });
            client_handles.push(handle);
        }
    
        let client_results = join_all(client_handles).await;
    
        for result in client_results {
            if let Err(e) = result {
                eprintln!("Client task failed: {:?}", e);
            }
        }
    
        let server_result = server_handle.await;
        if let Err(e) = server_result {
            eprintln!("Server task failed: {:?}", e);
        }
    }

    #[tokio::test]
    async fn single_connection() {
        let server_handle = tokio::spawn(async {
            impl_1::server(1).await.unwrap();
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let client_handle = tokio::spawn(async {
            impl_1::client().await.unwrap();
        });

        let _ = tokio::join!(server_handle, client_handle);
    }

}
