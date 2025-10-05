use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Protocol {
    TCP,
    UDP,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Protocol::TCP => write!(f, "TCP"),
            Protocol::UDP => write!(f, "UDP"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortState {
    Open,
    Filtered,
}

impl fmt::Display for PortState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PortState::Open => write!(f, "OPEN"),
            PortState::Filtered => write!(f, "FILTERED"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsInfo {
    pub subject: String,
    pub issuer: String,
    pub sans: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub port: u16,
    pub protocol: Protocol,
    pub state: PortState,
    pub service: Option<String>,
    pub version: Option<String>,
    pub banner: Option<String>,
    pub tls_info: Option<TlsInfo>,
    pub confidence: f32,
}

impl ServiceInfo {
    pub fn new(port: u16, protocol: Protocol, state: PortState) -> Self {
        Self {
            port,
            protocol,
            state,
            service: None,
            version: None,
            banner: None,
            tls_info: None,
            confidence: 0.0,
        }
    }

    pub fn with_service(mut self, service: String, confidence: f32) -> Self {
        self.service = Some(service);
        self.confidence = confidence;
        self
    }

    pub fn with_version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    pub fn with_banner(mut self, banner: String) -> Self {
        self.banner = Some(banner);
        self
    }

    pub fn with_tls_info(mut self, tls_info: TlsInfo) -> Self {
        self.tls_info = Some(tls_info);
        self
    }

    pub fn display_service(&self) -> String {
        let mut parts = Vec::new();

        if let Some(ref service) = self.service {
            parts.push(service.clone());
        } else {
            parts.push("unknown".to_string());
        }

        if let Some(ref version) = self.version {
            parts.push(format!("v{}", version));
        }

        parts.join(" ")
    }

    pub fn display_full(&self) -> String {
        let mut output = format!(
            "{} Port {} ({}) - {}",
            match self.protocol {
                Protocol::TCP => "TCP",
                Protocol::UDP => "UDP",
            },
            self.port,
            match self.state {
                PortState::Open => "OPEN",
                PortState::Filtered => "FILTERED",
            },
            self.display_service()
        );

        if let Some(ref banner) = self.banner {
            output.push_str(&format!(" | Banner: {}", banner));
        }

        if let Some(ref tls) = self.tls_info {
            output.push_str(&format!(" | TLS: {} (issued by {})", tls.subject, tls.issuer));
        }

        if self.confidence > 0.0 {
            output.push_str(&format!(" [confidence: {:.0}%]", self.confidence * 100.0));
        }

        output
    }
}
