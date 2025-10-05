use std::io::Read;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct SshBanner {
    pub version: String,
    pub software: String,
    pub comments: Option<String>,
}

/// Reads SSH banner from an open SSH port
/// SSH servers send their banner immediately upon connection
pub fn probe_ssh(addr: &str, port: u16) -> Option<SshBanner> {
    let socket_str = format!("{}:{}", addr, port);
    let socket_addr: SocketAddr = socket_str.parse().ok()?;

    let mut stream = TcpStream::connect_timeout(&socket_addr, Duration::from_millis(200)).ok()?;
    stream.set_read_timeout(Some(Duration::from_millis(500))).ok()?;

    // SSH servers send banner immediately, no need to send anything
    let mut buffer = [0u8; 256];
    let n = stream.read(&mut buffer).unwrap_or(0);

    if n == 0 {
        return None;
    }

    let banner_text = String::from_utf8_lossy(&buffer[..n]).to_string();
    parse_ssh_banner(&banner_text)
}

/// Parses SSH banner string
/// Format: SSH-protoversion-softwareversion [SP comments] CR LF
/// Example: SSH-2.0-OpenSSH_8.2p1 Ubuntu-4ubuntu0.5
fn parse_ssh_banner(banner: &str) -> Option<SshBanner> {
    let line = banner.lines().next()?.trim();

    if !line.starts_with("SSH-") {
        return None;
    }

    // Split on spaces to separate version from comments
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    let version_part = parts[0];
    let comments = parts.get(1).map(|s| s.to_string());

    // Parse version part: SSH-2.0-OpenSSH_8.2p1
    let version_parts: Vec<&str> = version_part.splitn(3, '-').collect();
    if version_parts.len() < 3 {
        return None;
    }

    let version = format!("{}-{}", version_parts[0], version_parts[1]);
    let software = version_parts[2].to_string();

    Some(SshBanner {
        version,
        software,
        comments,
    })
}

/// Checks if a port is likely SSH based on port number
pub fn is_likely_ssh_port(port: u16) -> bool {
    port == 22 || port == 2222
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ssh_banner() {
        let banner = "SSH-2.0-OpenSSH_8.2p1 Ubuntu-4ubuntu0.5\r\n";
        let parsed = parse_ssh_banner(banner).unwrap();
        assert_eq!(parsed.version, "SSH-2.0");
        assert_eq!(parsed.software, "OpenSSH_8.2p1");
        assert_eq!(parsed.comments, Some("Ubuntu-4ubuntu0.5".to_string()));
    }

    #[test]
    fn test_parse_ssh_banner_no_comments() {
        let banner = "SSH-2.0-dropbear_2019.78\n";
        let parsed = parse_ssh_banner(banner).unwrap();
        assert_eq!(parsed.version, "SSH-2.0");
        assert_eq!(parsed.software, "dropbear_2019.78");
        assert_eq!(parsed.comments, None);
    }

    #[test]
    fn test_is_likely_ssh_port() {
        assert!(is_likely_ssh_port(22));
        assert!(is_likely_ssh_port(2222));
        assert!(!is_likely_ssh_port(80));
    }
}
