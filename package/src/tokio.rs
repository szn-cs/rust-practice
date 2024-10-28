// Tokio includes a set of libraries for network/web service development; as well as, std async version

use bytes::Bytes;
use mini_redis::{Command, Connection, Frame};
use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;
use std::result;
use std::sync::{Arc, Mutex, MutexGuard};
use tokio::net::{TcpListener, TcpStream};

type Result<T> = result::Result<T, Box<dyn Error>>;

type DATABASE = Arc<Mutex<HashMap<String, Bytes>>>;

mod impl_1 {
    use super::*;

    #[tokio::main]
    pub async fn server() -> Result<()> {
        let address = "127.0.0.1:8080";
        let listener = TcpListener::bind(address).await?;
        println!("• listening {}", address);

        let db = Arc::new(Mutex::new(HashMap::new()));

        loop {
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

        Ok(())
    }

    #[tokio::main]
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

    #[test]
    fn tokio() {
        use mini_redis::client::Client;
        use std::thread::{scope, sleep, spawn};
        use std::time::Duration;

        scope(|s| {
            s.spawn(|| {
                impl_1::server();
            });

            sleep(Duration::from_millis(100));

            for _ in 0..100 {
                s.spawn(|| {
                    impl_1::client();
                });
            }
        });
    }
}
