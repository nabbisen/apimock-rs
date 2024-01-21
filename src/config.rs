use std::fs;
use std::collections::HashMap;
use std::path::Path;
use toml;
use serde_json::Value;
use json5;
use console::style;

use crate::LISTEN_PORT;

#[derive(Clone, Default)]
pub struct Config {
    pub port: u16,
    pub data_dir: Option<String>,
    pub always: Option<String>,
    pub path_prefix: Option<String>,
    pub paths: Option<HashMap<String, String>>,
}

const CONFIG_SECTION_GENERAL: &str = "general";
const CONFIG_SECTION_URL: &str = "url";
const CONFIG_KEY_PATH_PREFIX: &str = "path_prefix";

pub fn config(path: &str) -> Config {
    let mut config = default_config();

    let toml_string = fs::read_to_string(path).expect("No toml config file");
    let toml_content: toml::Value = toml::from_str(&toml_string).expect("Invalid toml file");
    let general_config = toml_content.get(CONFIG_SECTION_GENERAL).expect(format!("[{}] section missing", CONFIG_SECTION_GENERAL).as_str()).as_table().expect(format!("Invalid [{}] section", CONFIG_SECTION_GENERAL).as_str());
    for (key, value) in general_config {
        match key.as_str() {
            "port" => {
                match value.as_integer() {
                    Some(port) => {
                        config.port = port as u16
                    },
                    _ => ()
                }
            },
            "data_dir" => {
                match value.as_str() {
                    Some(data_dir) => {
                        println!("[data_dir] {}", data_dir);
                        config.data_dir = Some(data_dir.to_owned())
                    },
                    _ => ()
                }
            },
            "always" => {
                match value.as_str() {
                    Some(always) => {
                        let _ = json5::from_str::<Value>(&always).expect("Invalid always value");
                        config.always = Some(always.to_owned());
                        println!("[always] is specified");
                        return config;
                    },
                    _ => ()
                }
            },
            _ => (),
        }
    }

    let url_config = toml_content.get(CONFIG_SECTION_URL).expect(format!("[{}] section missing", CONFIG_SECTION_GENERAL).as_str()).as_table().expect(format!("Invalid [{}] section", CONFIG_SECTION_URL).as_str());
    let path_prefix = match url_config.get(CONFIG_KEY_PATH_PREFIX) {
        Some(got) => {
            match got.as_str() {
                Some(s) => {
                    println!("[path_prefix] {}", s);
                    s
                },
                _ => ""
            }
        },
        _ => "",
    };
    config.path_prefix = Some(path_prefix.to_owned());
    for (key, value) in url_config {
        match key.as_str() {
            "paths" => {
                let mut paths = HashMap::<String, String>::new();
                let p = value.as_table().expect("Invalid paths entries");
                for (path, json_file) in p {
                    let json_file = json_file.as_str().expect(format!("{} is invalid", json_file).as_str());
                    let json_path = Path::new(&config.data_dir.clone().unwrap()).join(json_file).display().to_string();
                    let _ = fs::metadata(&json_path).expect(format!("{} is missing", json_path).as_str());

                    let full_path = format!("/{}/{}/", path_prefix, path.to_string()).replace("//", "/");
                    let full_path_wo_trailing_slash = &full_path[..full_path.len() - 1];

                    paths.insert(full_path_wo_trailing_slash.to_owned(), json_path.to_owned());
                    println!("[path] {} => {}", style(full_path_wo_trailing_slash).yellow(), style(json_path).green());
                }
                config.paths = Some(paths);
            },
            _ => (),
        }
    }

    config
}

fn default_config() -> Config {
    let mut config = Config::default();
    config.port = LISTEN_PORT;
    config
}