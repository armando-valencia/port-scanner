use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

/// Attempts to connect to the given address and port,
/// returning the first line of the service banner (if any).
pub fn grab_banner(addr: &str, port: u16) -> Option<String> {
    let socket_str = format!("{}:{}", addr, port);
    let socket_addr: SocketAddr = socket_str.parse().ok()?;

    // Connect with a timeout
    let mut stream = TcpStream::connect_timeout(&socket_addr, Duration::from_millis(50)).ok()?;

    // Set a read timeout to avoid blocking indefinitely
    stream.set_read_timeout(Some(Duration::from_millis(100))).ok()?;

    // Send a GET request to see if port is active
    let _ = stream.write_all(b"GET / HTTP/1.0\r\n\r\n");

    // Read up to 128 bytes of the response
    let mut buf = [0u8; 128];
    let n = stream.read(&mut buf).unwrap_or(0);
    if n == 0 {
        return None;
    }

    // Convert bytes to string and return the first line
    let text = String::from_utf8_lossy(&buf[..n]).to_string();
    text.lines().next().map(|line| line.to_string())
}
