use std::io::prelude::*;
use std::net::{self, Shutdown, TcpStream};

pub fn sync_request() -> std::io::Result<String> {
    let host = "example.com";
    let path = "/";
    let port = 80;
    let mut socket = TcpStream::connect((host, port))?;

    let request = format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n",);
    socket.write_all(request.as_bytes())?;
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response)?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_connection() {
        let r = sync_request();
        println!("{}", r.unwrap());
    }
}
