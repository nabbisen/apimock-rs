use serde::Deserialize;

/// verbose logs
#[derive(Clone, Deserialize)]
pub struct ListenerConfig {
    pub ip_address: String,
    pub port: u16,
}
