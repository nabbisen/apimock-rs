use constant::*;
use listener_config::ListenerConfig;
use log_config::LogConfig;
use serde::Deserialize;
use service_config::ServiceConfig;
use toml;

use std::fs;

use super::server::routing::rule_set::RuleSet;

pub mod constant;
pub mod listener_config;
pub mod log_config;
pub mod service_config;

/// app config
#[derive(Clone, Deserialize)]
pub struct Config {
    #[serde(skip)]
    file_path: Option<String>,

    pub listener: ListenerConfig,
    pub log: LogConfig,
    pub service: ServiceConfig,
}

/// app config
impl Config {
    /// create new instance
    pub fn new(config_file_path: Option<&String>) -> Self {
        let mut config = if let Some(config_file_path) = config_file_path {
            log::info!("[config] {}\n", config_file_path);

            let toml_string = fs::read_to_string(config_file_path.as_str()).unwrap();
            let mut config: Config = match toml::from_str(&toml_string) {
                Ok(x) => x,
                Err(err) => panic!("{}: Invalid toml content\n({})", config_file_path, err),
            };

            config.file_path = Some(config_file_path.to_owned());

            config
        } else {
            Config::default()
        };

        config.service.rule_sets = config
            .service
            .rule_sets_file_paths
            .iter()
            .map(|x| RuleSet::new(x))
            .collect();

        if !config.validate() {
            panic!("failed to start up due to invalid config");
        }

        log::info!("{}", config);

        config
    }

    /// address listened to
    pub fn listener_address(&self) -> String {
        format!("{}:{}", self.listener.ip_address, self.listener.port)
    }

    /// update `fallback_respond_dir`
    // pub fn update_fallback_respond_dir(&mut self, data_dir: &str, old_data_dir: &str) {}

    /// validate settings in app config
    ///
    /// note: as to ListenerConfig validation, tcp listener is expected to run afterward instead
    /// note: none requires validation in LogConfig
    fn validate(&self) -> bool {
        self.service.validate()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            file_path: None,
            listener: ListenerConfig {
                ip_address: LISTENER_DEFAULT_IP_ADDRESS.to_owned(),
                port: LISTENER_DEFAULT_PORT,
            },
            log: LogConfig::default(),
            service: ServiceConfig::default(),
        }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "{}", self.log);
        let _ = writeln!(f, "{}", PRINT_DELIMITER);
        let _ = write!(f, "{}", self.service);
        Ok(())
    }
}
