use constant::*;
use listener_config::ListenerConfig;
use log_config::{LogConfig, VerboseConfig};
use serde::Deserialize;
use service_config::ServiceConfig;
use toml;

use std::fs;

use super::server::routing::rule_set::RuleSet;

pub mod constant;
pub mod listener_config;
pub mod log_config;
pub mod service_config;
mod util;

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
        let config = if let Some(config_file_path) = config_file_path {
            log::info!("[config] {}\n", config_file_path);

            let toml_string = fs::read_to_string(config_file_path.as_str()).unwrap();
            let mut config: Config = match toml::from_str(&toml_string) {
                Ok(x) => x,
                Err(err) => panic!("{}: Invalid toml content\n({})", config_file_path, err),
            };

            config.service.rule_sets = config
                .service
                .rule_sets_file_paths
                .iter()
                .map(|x| RuleSet::new(x))
                .collect();

            config.file_path = Some(config_file_path.to_owned());

            config
        } else {
            Config::default()
        };

        config.validate();

        util::print(&config);
        config
    }

    /// address listened to
    pub fn listener_address(&self) -> String {
        format!("{}:{}", self.listener.ip_address, self.listener.port)
    }

    /// update `fallback_response_dir`
    // pub fn update_fallback_response_dir(&mut self, data_dir: &str, old_data_dir: &str) {}

    /// validate settings in app config
    fn validate(&self) {
        // todo: validate rule_sets
        // self.service.rule_sets.iter().for_each(|x| x.);

        // if self.always.is_none()
        //     && (self.paths.is_none() || self.paths.clone().unwrap().len() == 0)
        //     && self.dyn_data_dir.is_none()
        // {
        //     panic!("paths not defined");
        // }

        // let _ = match self.data_dir_query_path.clone() {
        //     Some(data_dir_query_path) if data_dir_query_path == "" => {
        //         panic!("data_dir_query_path is set but empty");
        //     }
        //     _ => (),
        // };

        // if let Some(paths) = &self.paths {
        //     for (path, path_config) in paths {
        //         if !path_config.data_src.is_none() && !path_config.data_text.is_none() {
        //             panic!("can't define src and text on path: {}", path);
        //         }
        //     }
        // }
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
