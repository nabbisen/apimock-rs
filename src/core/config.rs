use constant::*;
use serde::Deserialize;
use toml;

use std::fs;

use super::routing::rule_set::RuleSet;

pub mod constant;
mod util;

/// app config
#[derive(Clone, Deserialize)]
pub struct Config {
    #[serde(skip)]
    file_path: Option<String>,

    // [listener]
    #[serde(default)]
    pub ip_address: String,
    #[serde(default)]
    pub port: u16,

    // [log]
    #[serde(default)]
    pub verbose: Verbose,

    // [service]
    #[serde(default)]
    pub default_response_dir: String,

    // [routing]
    #[serde(default)]
    #[serde(rename = "rule_sets")]
    pub rule_sets_file_paths: Vec<String>,
    #[serde(skip)]
    pub rule_sets: Vec<RuleSet>,
}

/// verbose logs
#[derive(Clone, Default, Deserialize)]
pub struct Verbose {
    pub header: bool,
    pub body: bool,
}

/// app config
impl Config {
    /// create new instance
    pub fn new(config_filepath: Option<&String>) -> Self {
        let config = if let Some(config_filepath) = config_filepath {
            log::info!("[config] {}\n", config_filepath);

            let toml_string = fs::read_to_string(config_filepath.as_str()).unwrap();
            let mut config: Config = match toml::from_str(&toml_string) {
                Ok(x) => x,
                Err(err) => panic!("{}: Invalid toml content\n({})", config_filepath, err),
            };

            config.rule_sets = config
                .rule_sets_file_paths
                .iter()
                .map(|x| RuleSet::new(x))
                .collect();

            config.file_path = Some(config_filepath.to_owned());

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
        format!("{}:{}", self.ip_address, self.port)
    }

    /// update `default_response_dir`
    // pub fn update_default_response_dir(&mut self, data_dir: &str, old_data_dir: &str) {}

    /// validate settings in app config
    fn validate(&self) {
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
            ip_address: LISTENER_DEFAULT_IP_ADDRESS.to_owned(),
            port: LISTENER_DEFAULT_PORT,
            verbose: Verbose::default(),
            default_response_dir: SERVICE_DEFAULT_DEFAULT_RESPONSE_DIR.to_owned(),
            rule_sets_file_paths: vec![],
            rule_sets: vec![],
        }
    }
}
