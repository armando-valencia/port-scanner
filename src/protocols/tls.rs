use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use crate::service_info::TlsInfo;

/// Attempts a basic TLS detection by looking for TLS handshake response
/// NOTE: This is a simplified version. For full certificate extraction,
/// you would need to add rustls or native-tls as dependencies.
pub fn probe_tls(addr: &str, port: u16) -> Option<TlsInfo> {
    let socket_addr = format!("{}:{}", addr, port);

    // Connect to the server
    let mut stream = TcpStream::connect_timeout(
        &socket_addr.parse().ok()?,
        Duration::from_millis(1000)
    ).ok()?;

    stream.set_read_timeout(Some(Duration::from_millis(2000))).ok()?;
    stream.set_write_timeout(Some(Duration::from_millis(1000))).ok()?;

    // Send a TLS ClientHello (simplified)
    // This is a minimal TLS 1.2 ClientHello packet
    let client_hello = create_simple_client_hello();

    if stream.write_all(&client_hello).is_err() {
        return None;
    }

    // Flush to ensure data is sent
    let _ = stream.flush();

    // Try to read ServerHello response
    let mut buffer = [0u8; 2048];
    let n = stream.read(&mut buffer).unwrap_or(0);

    if n > 5 && buffer[0] == 0x16 {
        // 0x16 = Handshake record type in TLS
        // We detected TLS response - extract more info if possible
        Some(TlsInfo {
            subject: "TLS Service Detected".to_string(),
            issuer: "Certificate parsing requires rustls".to_string(),
            sans: Vec::new(),
        })
    } else if n > 0 {
        // Got some response but not TLS handshake
        // Still might be HTTPS, just didn't parse correctly
        Some(TlsInfo {
            subject: "Possible TLS Service".to_string(),
            issuer: "Non-standard response".to_string(),
            sans: Vec::new(),
        })
    } else {
        None
    }
}

/// Creates a minimal TLS ClientHello for detection purposes
fn create_simple_client_hello() -> Vec<u8> {
    // A proper minimal TLS 1.2 ClientHello
    // This is the smallest valid ClientHello that will trigger a ServerHello response
    vec![
        // TLS Record Header
        0x16,       // Content Type: Handshake
        0x03, 0x01, // Version: TLS 1.0 (for compatibility)
        0x00, 0x3e, // Length: 62 bytes

        // Handshake Header
        0x01,             // Handshake Type: ClientHello
        0x00, 0x00, 0x3a, // Length: 58 bytes

        // ClientHello
        0x03, 0x03, // Version: TLS 1.2

        // Random (32 bytes) - Using zeros for simplicity
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,

        0x00, // Session ID Length: 0

        // Cipher Suites
        0x00, 0x02, // Length: 2 bytes (1 cipher suite)
        0x00, 0x3c, // TLS_RSA_WITH_AES_128_CBC_SHA256

        // Compression Methods
        0x01, // Length: 1
        0x00, // Compression: null

        // Extensions Length: 0
        0x00, 0x00,
    ]
}

/// Checks if a port is likely to use TLS based on port number
pub fn is_likely_tls_port(port: u16) -> bool {
    matches!(port, 443 | 465 | 587 | 636 | 993 | 995 | 8443)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_likely_tls_port() {
        assert!(is_likely_tls_port(443));
        assert!(is_likely_tls_port(993));
        assert!(!is_likely_tls_port(80));
        assert!(!is_likely_tls_port(22));
    }
}
