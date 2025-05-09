use std::net::UdpSocket;
use std::time::Duration;
use std::io::ErrorKind;

/// Sends a zero-byte UDP packet to the given address and port.
/// Returns `true` if the port is likely open or filtered (no ICMP unreachable),
/// or `false` if a "ConnectionRefused" ICMP message is received (port closed).
pub fn scan_udp(addr: &str, port: u16, timeout_ms: u64) -> bool {
    let socket_addr = (addr, port);

    // Bind a local ephemeral UDP socket
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return false,
    };

    // Set a read timeout to avoid blocking indefinitely
    if socket.set_read_timeout(Some(Duration::from_millis(timeout_ms)))
    .is_err() {
        return false;
    }

    // Send an empty datagram to the target
    let _ = socket.send_to(&[], socket_addr);

    // Try to receive a reply or ICMP error
    let mut buf = [0u8; 1];
    match socket.recv_from(&mut buf) {
        Ok(_) => true,  // Received some packet => open/filtered
        Err(ref e) if e.kind() == ErrorKind::WouldBlock => true,  // Timeout => open/filtered
        Err(ref e) if e.kind() == ErrorKind::ConnectionRefused => false,  // ICMP unreachable => closed
        Err(_) => false,  // treat as closed
    }
}