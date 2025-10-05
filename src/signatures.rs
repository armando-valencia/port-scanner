use serde::{Deserialize, Serialize};
use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BannerPattern {
    pub pattern: String,
    pub service: String,
    pub product: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpServerPattern {
    pub pattern: String,
    pub service: String,
    pub product: String,
    pub confidence: f32,
    pub version_group: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureDatabase {
    pub banner_patterns: Vec<BannerPattern>,
    pub http_server_patterns: Vec<HttpServerPattern>,
    pub port_hints: HashMap<String, String>,
}

pub struct SignatureMatcher {
    database: SignatureDatabase,
    banner_regexes: Vec<(Regex, BannerPattern)>,
    http_regexes: Vec<(Regex, HttpServerPattern)>,
}

#[derive(Debug, Clone)]
pub struct Match {
    pub service: String,
    pub product: String,
    pub version: Option<String>,
    pub confidence: f32,
}

impl SignatureMatcher {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let database: SignatureDatabase = serde_json::from_str(&content)?;

        // Compile banner regex patterns
        let mut banner_regexes = Vec::new();
        for pattern in &database.banner_patterns {
            if let Ok(regex) = Regex::new(&pattern.pattern) {
                banner_regexes.push((regex, pattern.clone()));
            }
        }

        // Compile HTTP server regex patterns
        let mut http_regexes = Vec::new();
        for pattern in &database.http_server_patterns {
            if let Ok(regex) = Regex::new(&pattern.pattern) {
                http_regexes.push((regex, pattern.clone()));
            }
        }

        Ok(Self {
            database,
            banner_regexes,
            http_regexes,
        })
    }

    pub fn match_banner(&self, banner: &str) -> Option<Match> {
        for (regex, pattern) in &self.banner_regexes {
            if regex.is_match(banner) {
                return Some(Match {
                    service: pattern.service.clone(),
                    product: pattern.product.clone(),
                    version: None,
                    confidence: pattern.confidence,
                });
            }
        }
        None
    }

    pub fn match_http_server(&self, server_header: &str) -> Option<Match> {
        for (regex, pattern) in &self.http_regexes {
            if let Some(captures) = regex.captures(server_header) {
                let version = if let Some(group) = pattern.version_group {
                    captures.get(group).map(|m| m.as_str().to_string())
                } else {
                    None
                };

                return Some(Match {
                    service: pattern.service.clone(),
                    product: pattern.product.clone(),
                    version,
                    confidence: pattern.confidence,
                });
            }
        }
        None
    }

    pub fn get_port_hint(&self, port: u16) -> Option<String> {
        self.database
            .port_hints
            .get(&port.to_string())
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_banner_matching() {
        let matcher = SignatureMatcher::load("signatures.json").unwrap();

        let ssh_banner = "SSH-2.0-OpenSSH_8.2p1 Ubuntu-4ubuntu0.5";
        let result = matcher.match_banner(ssh_banner);
        assert!(result.is_some());
        let m = result.unwrap();
        assert_eq!(m.service, "ssh");
        assert_eq!(m.product, "OpenSSH");
    }

    #[test]
    fn test_http_server_matching() {
        let matcher = SignatureMatcher::load("signatures.json").unwrap();

        let nginx_header = "nginx/1.18.0";
        let result = matcher.match_http_server(nginx_header);
        assert!(result.is_some());
        let m = result.unwrap();
        assert_eq!(m.service, "http");
        assert_eq!(m.product, "nginx");
        assert_eq!(m.version, Some("1.18.0".to_string()));
    }
}
