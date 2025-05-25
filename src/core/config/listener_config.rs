use serde::Deserialize;

use super::constant::{LISTENER_DEFAULT_IP_ADDRESS, LISTENER_DEFAULT_PORT};

/// verbose logs
#[derive(Clone, Deserialize)]
pub struct ListenerConfig {
    pub ip_address: String,
    pub port: u16,
}

impl Default for ListenerConfig {
    fn default() -> Self {
        Self {
            ip_address: LISTENER_DEFAULT_IP_ADDRESS.to_owned(),
            port: LISTENER_DEFAULT_PORT,
        }
    }
}
