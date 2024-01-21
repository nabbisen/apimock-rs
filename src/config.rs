use std::fs;
use std::collections::HashMap;
use toml;

use crate::LISTEN_PORT;
use crate::CONFIG_FILENAME;

#[derive(Default, Debug)]
pub struct Config {
    pub port: u16,
    pub data_dir: Option<String>,
    pub always: Option<String>,
    pub path_prefix: Option<String>,
    pub paths: Option<HashMap<String, String>>,
}

const CONFIG_SECTION_GENERAL: &str = "general";
const CONFIG_SECTION_URL: &str = "url";

pub fn config() -> Config {
    let mut config = default_config();

    let toml_string = fs::read_to_string(CONFIG_FILENAME).expect("No toml config file");
    let toml_content: toml::Value = toml::from_str(&toml_string).expect("Invalid toml file");
    let general_config = toml_content.get(CONFIG_SECTION_GENERAL).expect(format!("[{}] section missing", CONFIG_SECTION_GENERAL).as_str()).as_table().expect(format!("Invalid [{}] section", CONFIG_SECTION_GENERAL).as_str());
    for (key, value) in general_config {
        println!("{}: {:?}", key, value); // todo
        match key.as_str() {
            "port" => config.port = value.as_integer().unwrap() as u16,
            "data_dir" => config.data_dir = Some(value.as_str().unwrap().to_owned()),
            "always" => config.always = Some(value.as_str().unwrap().to_owned()),
            _ => (),
        }
    }
    let url_config = toml_content.get(CONFIG_SECTION_URL).expect(format!("[{}] section missing", CONFIG_SECTION_GENERAL).as_str()).as_table().expect(format!("Invalid [{}] section", CONFIG_SECTION_URL).as_str());
    for (key, value) in url_config {
        println!("{}: {:?}", key, value); // todo
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
    }

    config
}

fn default_config() -> Config {
    let mut config = Config::default();
    config.port = LISTEN_PORT;
    config
}