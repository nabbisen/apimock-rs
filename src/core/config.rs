use constant::*;
use listener_config::ListenerConfig;
use log_config::LogConfig;
use serde::Deserialize;
use service_config::ServiceConfig;
use toml;

use std::{fs, path::Path};

use crate::core::server::middleware::Middleware;

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
    pub fn new(config_file_path: Option<&String>, middleware_file_path: Option<&String>) -> Self {
        let mut ret = Self::init(config_file_path);

        ret.set_rule_sets();
        ret.set_default_middleware_file_path_if_specified(middleware_file_path);
        match ret.middlewares_from_file_paths() {
            Ok(x) => {
                if !x.is_empty() {
                    log::info!("Middleware is activated: {} file(s)", x.len());
                }
                ret.service.middlewares = x
            }
            Err(x) => panic!("{}", x),
        }

        if !ret.validate() {
            panic!("failed to start up due to invalid config");
        }

        log::info!("{}", ret);

        ret
    }

    /// initialize
    fn init(config_file_path: Option<&String>) -> Self {
        let ret = if let Some(config_file_path) = config_file_path {
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

        ret
    }

    /// set rule sets from rule sets file paths
    fn set_rule_sets(&mut self) {
        self.service.rule_sets = self
            .service
            .rule_sets_file_paths
            .iter()
            .enumerate()
            .map(|(rule_set_idx, rule_set_file_path)| {
                RuleSet::new(rule_set_file_path, rule_set_idx)
            })
            .collect();
    }

    /// set middlewares file paths if no file path is specified
    fn set_default_middleware_file_path_if_specified(
        &mut self,
        middleware_file_path: Option<&String>,
    ) {
        if let Some(middlewares_file_paths) = self.service.middlewares_file_paths.as_ref() {
            if !middlewares_file_paths.is_empty() {
                return;
            }
        }

        let _ = match middleware_file_path {
            Some(x) => {
                if !Path::new(x).exists() {
                    panic!("default middleware is specified but doesn't exist: {}", x);
                }
                self.service.middlewares_file_paths = Some(vec![x.to_owned()]);
            }
            None => (),
        };
    }

    /// set middlewares from middlewares file paths
    fn middlewares_from_file_paths(&mut self) -> Result<Vec<Middleware>, String> {
        match self.service.middlewares_file_paths.as_ref() {
            Some(x) => x
                .iter()
                .map(|middlware_file_path| Middleware::new(middlware_file_path.as_str()))
                .collect(),
            None => Ok(vec![]),
        }
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
