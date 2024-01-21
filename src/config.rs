use std::fs;
use std::collections::HashMap;
use toml;

use crate::LISTEN_PORT;
use crate::CONFIG_FILENAME;

#[derive(Debug)]
pub struct Config {
    port: u16,
    always: Option<String>,
    path_prefix: Option<String>,
    paths: Option<HashMap<String, String>>,
}

pub fn config() -> Config {
    let mut config = default_config();

    let toml_string = fs::read_to_string(CONFIG_FILENAME).expect("No toml config file");
    let toml_content: toml::Value = toml::from_str(&toml_string).expect("Invalid toml file");
    let general_config = toml_content.get("general").expect("[general] section missing").as_table().expect("Invalid [general] section");
    for (key, value) in general_config {
        match key.as_str() {
            "port" => config.port = value.as_integer().unwrap() as u16,
            "always" => config.always = Some(value.as_str().unwrap().to_owned()),
            _ => (),
        }
        println!("{}: {:?}", key, value); // todo
    }
    let url_config = toml_content.get("url").expect("[url] section missing").as_table().expect("Invalid [url] section");
    for (key, value) in url_config {
        match key.as_str() {
            "path_prefix" => config.path_prefix = Some(value.as_str().unwrap().to_owned()),
            "paths" => {
                let mut paths = HashMap::<String, String>::new();
                let p = value.as_table().expect("Invalid paths entries");
                for (k, v) in p {
                    paths.insert(k.to_string(), v.to_string());
                }
                config.paths = Some(paths);
            },
            _ => (),
        }
        println!("{}: {:?}", key, value); // todo
    }

    config
}

fn default_config() -> Config {
    Config {
        port: LISTEN_PORT,
        always: None,
        path_prefix: Some(String::new()),
        paths: None,
    }
}