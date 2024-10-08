use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};

fn sync_request() -> std::io::Result<String> {
    let host = "8.8.8.8";
    let path = "";
    let mut socket = TcpStream::connect((host, 80))?;

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes())?;
    socket.shutdown(Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response)?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_connection() {
        let _ = sync_request();
    }
}
