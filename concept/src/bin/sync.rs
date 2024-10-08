use concept::sync::sync_request;

fn main() {
    let r = sync_request().unwrap();
    println!("main ended {}", r);

    // let r = get_google_homepage().unwrap();
    // println!("{}", r);
}

use std::io::prelude::*;
use std::net::{self, Shutdown, TcpStream};

pub fn get_google_homepage() -> std::io::Result<String> {
    let host = "google.com";
    let path = "/";
    let port = 80; // Default HTTP port

    let mut socket = TcpStream::connect((host, port))?;

    let request = format!(
        "GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n",
        path = path,
        host = host
    );

    socket.write_all(request.as_bytes())?;
    socket.shutdown(Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response)?;

    Ok(response)
}
