use crate::service_info::{ServiceInfo, Protocol, PortState};
use crate::signatures::SignatureMatcher;
use crate::protocols::{http, tls, ssh, smtp_ftp};

/// Main fingerprinting orchestrator
/// Takes an open port and attempts to identify the service running on it
pub fn fingerprint_service(
    addr: &str,
    port: u16,
    protocol: Protocol,
    matcher: &SignatureMatcher,
) -> ServiceInfo {
    let mut info = ServiceInfo::new(port, protocol.clone(), PortState::Open);

    // Start with port-based hint
    if let Some(hint) = matcher.get_port_hint(port) {
        info = info.with_service(hint.clone(), 0.3); // Low confidence, just a guess
    }

    match protocol {
        Protocol::TCP => fingerprint_tcp(addr, port, matcher, info),
        Protocol::UDP => {
            // UDP fingerprinting is limited
            info.state = PortState::Filtered;
            info
        }
    }
}

/// Fingerprint TCP services
fn fingerprint_tcp(
    addr: &str,
    port: u16,
    matcher: &SignatureMatcher,
    mut info: ServiceInfo,
) -> ServiceInfo {
    // Try SSH first (common and quick)
    if ssh::is_likely_ssh_port(port) || info.service.as_deref() == Some("ssh") {
        if let Some(ssh_banner) = ssh::probe_ssh(addr, port) {
            let full_banner = format!("{} {}", ssh_banner.software,
                ssh_banner.comments.as_deref().unwrap_or(""));

            // Try to match against signatures
            if let Some(matched) = matcher.match_banner(&full_banner) {
                info = info.with_service(matched.product.clone(), matched.confidence);
                if let Some(ver) = matched.version {
                    info = info.with_version(ver);
                }
            } else {
                info = info.with_service(ssh_banner.software.clone(), 0.8);
            }

            info = info.with_banner(full_banner);
            return info;
        }
    }

    // Try FTP
    if smtp_ftp::is_likely_ftp_port(port) || info.service.as_deref() == Some("ftp") {
        if let Some(greeting) = smtp_ftp::probe_ftp(addr, port) {
            info = info.with_banner(greeting.clone());

            if let Some(matched) = matcher.match_banner(&greeting) {
                info = info.with_service(matched.product.clone(), matched.confidence);
                if let Some(ver) = matched.version {
                    info = info.with_version(ver);
                }
            } else {
                info = info.with_service("FTP".to_string(), 0.7);
            }
            return info;
        }
    }

    // Try SMTP
    if smtp_ftp::is_likely_smtp_port(port) || info.service.as_deref() == Some("smtp") {
        if let Some(greeting) = smtp_ftp::probe_smtp(addr, port) {
            info = info.with_banner(greeting.clone());

            if let Some(matched) = matcher.match_banner(&greeting) {
                info = info.with_service(matched.product.clone(), matched.confidence);
                if let Some(ver) = matched.version {
                    info = info.with_version(ver);
                }
            } else {
                info = info.with_service("SMTP".to_string(), 0.7);
            }
            return info;
        }
    }

    // Try POP3
    if smtp_ftp::is_likely_pop3_port(port) || info.service.as_deref() == Some("pop3") {
        if let Some(greeting) = smtp_ftp::probe_pop3(addr, port) {
            info = info.with_banner(greeting.clone());

            if let Some(matched) = matcher.match_banner(&greeting) {
                info = info.with_service(matched.product.clone(), matched.confidence);
                if let Some(ver) = matched.version {
                    info = info.with_version(ver);
                }
            } else {
                info = info.with_service("POP3".to_string(), 0.7);
            }
            return info;
        }
    }

    // Try IMAP
    if smtp_ftp::is_likely_imap_port(port) || info.service.as_deref() == Some("imap") {
        if let Some(greeting) = smtp_ftp::probe_imap(addr, port) {
            info = info.with_banner(greeting.clone());

            if let Some(matched) = matcher.match_banner(&greeting) {
                info = info.with_service(matched.product.clone(), matched.confidence);
                if let Some(ver) = matched.version {
                    info = info.with_version(ver);
                }
            } else {
                info = info.with_service("IMAP".to_string(), 0.7);
            }
            return info;
        }
    }

    // Try TLS for HTTPS and other TLS services
    if tls::is_likely_tls_port(port) || info.service.as_deref() == Some("https") {
        if let Some(tls_info) = tls::probe_tls(addr, port) {
            info = info.with_tls_info(tls_info);

            // If we got TLS info, it's likely HTTPS
            if port == 443 {
                info = info.with_service("HTTPS".to_string(), 0.9);
            } else {
                info = info.with_service(format!("TLS (port {})", port), 0.8);
            }

            // Still try HTTP to get server info
            // Note: This would require HTTPS support in http.rs
            return info;
        }
    }

    // Try HTTP (should be tried after TLS for HTTPS ports)
    if port == 80 || port == 8080 || port == 8000 || info.service.as_deref() == Some("http") {
        if let Some(http_response) = http::probe_http(addr, port, false) {
            if let Some(server) = http::extract_server_info(&http_response) {
                if let Some(matched) = matcher.match_http_server(&server) {
                    info = info.with_service(matched.product.clone(), matched.confidence);
                    if let Some(ver) = matched.version {
                        info = info.with_version(ver);
                    }
                } else {
                    info = info.with_service("HTTP".to_string(), 0.7);
                }

                info = info.with_banner(http_response.status_line.clone());
            } else {
                info = info.with_service("HTTP".to_string(), 0.6);
                info = info.with_banner(http_response.status_line.clone());
            }
            return info;
        }
    }

    // If nothing worked, keep the port hint or mark as unknown
    if info.service.is_none() {
        info = info.with_service("unknown".to_string(), 0.1);
    }

    info
}
