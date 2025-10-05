use std::io::Read;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

/// Probes for SMTP greeting banner
/// SMTP servers send a 220 greeting immediately upon connection
pub fn probe_smtp(addr: &str, port: u16) -> Option<String> {
    read_greeting(addr, port)
}

/// Probes for FTP greeting banner
/// FTP servers send a 220 greeting immediately upon connection
pub fn probe_ftp(addr: &str, port: u16) -> Option<String> {
    read_greeting(addr, port)
}

/// Probes for POP3 greeting banner
/// POP3 servers send a +OK greeting immediately upon connection
pub fn probe_pop3(addr: &str, port: u16) -> Option<String> {
    read_greeting(addr, port)
}

/// Probes for IMAP greeting banner
/// IMAP servers send an untagged OK greeting immediately upon connection
pub fn probe_imap(addr: &str, port: u16) -> Option<String> {
    read_greeting(addr, port)
}

/// Generic function to read greeting from servers that speak first
fn read_greeting(addr: &str, port: u16) -> Option<String> {
    let socket_str = format!("{}:{}", addr, port);
    let socket_addr: SocketAddr = socket_str.parse().ok()?;

    let mut stream = TcpStream::connect_timeout(&socket_addr, Duration::from_millis(300)).ok()?;
    stream.set_read_timeout(Some(Duration::from_millis(1000))).ok()?;

    // These protocols send greeting immediately, no need to send anything
    let mut buffer = [0u8; 512];
    let n = stream.read(&mut buffer).unwrap_or(0);

    if n == 0 {
        return None;
    }

    let greeting = String::from_utf8_lossy(&buffer[..n]).to_string();
    let first_line = greeting.lines().next()?.trim().to_string();

    if !first_line.is_empty() {
        Some(first_line)
    } else {
        None
    }
}

/// Checks if a port is likely SMTP
pub fn is_likely_smtp_port(port: u16) -> bool {
    matches!(port, 25 | 465 | 587)
}

/// Checks if a port is likely FTP
pub fn is_likely_ftp_port(port: u16) -> bool {
    port == 21
}

/// Checks if a port is likely POP3
pub fn is_likely_pop3_port(port: u16) -> bool {
    matches!(port, 110 | 995)
}

/// Checks if a port is likely IMAP
pub fn is_likely_imap_port(port: u16) -> bool {
    matches!(port, 143 | 993)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_likely_smtp_port() {
        assert!(is_likely_smtp_port(25));
        assert!(is_likely_smtp_port(587));
        assert!(!is_likely_smtp_port(80));
    }

    #[test]
    fn test_is_likely_ftp_port() {
        assert!(is_likely_ftp_port(21));
        assert!(!is_likely_ftp_port(22));
    }

    #[test]
    fn test_is_likely_pop3_port() {
        assert!(is_likely_pop3_port(110));
        assert!(is_likely_pop3_port(995));
        assert!(!is_likely_pop3_port(143));
    }

    #[test]
    fn test_is_likely_imap_port() {
        assert!(is_likely_imap_port(143));
        assert!(is_likely_imap_port(993));
        assert!(!is_likely_imap_port(110));
    }
}
