use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status_line: String,
    pub server: Option<String>,
    pub headers: Vec<(String, String)>,
    pub body_preview: String,
}

/// Sends an HTTP HEAD request and parses the response headers
pub fn probe_http(addr: &str, port: u16, is_https: bool) -> Option<HttpResponse> {
    if is_https {
        // For HTTPS, we'll handle this in the TLS module
        return None;
    }

    let socket_str = format!("{}:{}", addr, port);
    let socket_addr: SocketAddr = socket_str.parse().ok()?;

    let mut stream = TcpStream::connect_timeout(&socket_addr, Duration::from_millis(200)).ok()?;
    stream.set_read_timeout(Some(Duration::from_millis(500))).ok()?;
    stream.set_write_timeout(Some(Duration::from_millis(200))).ok()?;

    // Send HTTP HEAD request
    let request = format!(
        "HEAD / HTTP/1.1\r\nHost: {}\r\nUser-Agent: port-scanner/0.1\r\nConnection: close\r\n\r\n",
        addr
    );

    stream.write_all(request.as_bytes()).ok()?;

    // Read response
    let mut buffer = [0u8; 4096];
    let n = stream.read(&mut buffer).unwrap_or(0);
    if n == 0 {
        return None;
    }

    let response_text = String::from_utf8_lossy(&buffer[..n]).to_string();
    parse_http_response(&response_text)
}

/// Parses HTTP response text into structured data
fn parse_http_response(response: &str) -> Option<HttpResponse> {
    let mut lines = response.lines();
    let status_line = lines.next()?.to_string();

    let mut headers = Vec::new();
    let mut server = None;

    for line in lines {
        if line.is_empty() {
            break; // End of headers
        }

        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim().to_string();
            let value = line[colon_pos + 1..].trim().to_string();

            if key.eq_ignore_ascii_case("server") {
                server = Some(value.clone());
            }

            headers.push((key, value));
        }
    }

    Some(HttpResponse {
        status_line,
        server,
        headers,
        body_preview: String::new(),
    })
}

/// Extract server software from HTTP response
pub fn extract_server_info(response: &HttpResponse) -> Option<String> {
    response.server.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_http_response() {
        let response = "HTTP/1.1 200 OK\r\nServer: nginx/1.18.0\r\nContent-Type: text/html\r\n\r\n";
        let parsed = parse_http_response(response).unwrap();
        assert_eq!(parsed.status_line, "HTTP/1.1 200 OK");
        assert_eq!(parsed.server, Some("nginx/1.18.0".to_string()));
    }
}
