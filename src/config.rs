use console::style;
use hyper::StatusCode;
use json5;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use toml;

use crate::LISTEN_PORT;

#[derive(Clone, Default)]
pub struct Config {
    pub port: u16,
    pub data_dir: Option<String>,
    pub always: Option<String>,
    pub path_prefix: Option<String>,
    pub paths: Option<HashMap<String, String>>,
    pub errors: Option<HashMap<u16, Vec<String>>>,
}

const CONFIG_SECTION_GENERAL: &str = "general";
const CONFIG_SECTION_URL: &str = "url";
const CONFIG_KEY_PATH_PREFIX: &str = "path_prefix";

pub fn config(path: &str) -> Config {
    let mut config = default_config();

    let toml_string = fs::read_to_string(path).expect("No toml config file");
    let toml_content: toml::Value = toml::from_str(&toml_string).expect("Invalid toml file");
    let general_config = toml_content
        .get(CONFIG_SECTION_GENERAL)
        .expect(format!("[{}] section missing", CONFIG_SECTION_GENERAL).as_str())
        .as_table()
        .expect(format!("Invalid [{}] section", CONFIG_SECTION_GENERAL).as_str());
    for (key, value) in general_config {
        match key.as_str() {
            "port" => match value.as_integer() {
                Some(port) => config.port = port as u16,
                _ => (),
            },
            "data_dir" => match value.as_str() {
                Some(data_dir) => {
                    println!("[data_dir] {}", data_dir);
                    config.data_dir = Some(data_dir.to_owned())
                }
                _ => (),
            },
            "always" => match value.as_str() {
                Some(always) => {
                    let _ = json5::from_str::<Value>(&always).expect("Invalid always value");
                    config.always = Some(always.to_owned());
                    println!("[always] is specified");
                    return config;
                }
                _ => (),
            },
            _ => (),
        }
    }

    let url_config = toml_content
        .get(CONFIG_SECTION_URL)
        .expect(format!("[{}] section missing", CONFIG_SECTION_GENERAL).as_str())
        .as_table()
        .expect(format!("Invalid [{}] section", CONFIG_SECTION_URL).as_str());
    let path_prefix = match url_config.get(CONFIG_KEY_PATH_PREFIX) {
        Some(got) => match got.as_str() {
            Some(s) => {
                println!("[path_prefix] {}", s);
                s
            }
            _ => "",
        },
        _ => "",
    };
    config.path_prefix = Some(path_prefix.to_owned());
    for (key, value) in url_config {
        match key.as_str() {
            "paths" => {
                config.paths = Some(config_url_paths(
                    value,
                    config.data_dir.clone().unwrap().as_str(),
                    path_prefix,
                ))
            }
            "errors" => config.errors = Some(config_url_errors(value, path_prefix)),
            _ => (),
        }
    }
    let duplicate = validate_paths(&config.paths, &config.errors);
    if 0 < duplicate.len() {
        panic!("Invalid path: {} is duplicate", duplicate);
    }

    config
}

fn default_config() -> Config {
    let mut config = Config::default();
    config.port = LISTEN_PORT;
    config
}

fn config_url_paths(
    value: &toml::Value,
    data_dir: &str,
    path_prefix: &str,
) -> HashMap<String, String> {
    let p = value.as_table().expect("Invalid paths entries");
    let paths = p
        .into_iter()
        .map(|(path, json_file)| config_url_path(path, json_file, data_dir, path_prefix))
        .collect::<HashMap<String, String>>();
    paths
}

fn config_url_path(
    path: &str,
    json_file: &toml::Value,
    data_dir: &str,
    path_prefix: &str,
) -> (String, String) {
    let json_file = json_file
        .as_str()
        .expect(format!("{} is invalid", json_file).as_str());
    let json_path = Path::new(data_dir).join(json_file).display().to_string();
    let _ = fs::metadata(&json_path).expect(format!("{} is missing", json_path).as_str());

    let full_path = format!("/{}/{}/", path_prefix, path.to_string()).replace("//", "/");
    let full_path_wo_trailing_slash = &full_path[..full_path.len() - 1];

    println!(
        "[path] {} => {}",
        style(full_path_wo_trailing_slash).yellow(),
        style(json_path.clone()).green()
    );
    (full_path_wo_trailing_slash.to_owned(), json_path.to_owned())
}

fn config_url_errors(value: &toml::Value, path_prefix: &str) -> HashMap<u16, Vec<String>> {
    let p = value.as_table().expect("Invalid url errors entries");
    let errors = p
        .into_iter()
        .filter_map(|(code, paths)| {
            let status_code = if let Ok(x) = code.parse::<u16>() {
                x
            } else {
                panic!("{}: Not numeric error code", code);
            };
            let status_code = if let Ok(x) = StatusCode::from_u16(status_code) {
                x
            } else {
                panic!("{}: Invalid error code", status_code);
            };
            if !StatusCode::is_client_error(&status_code)
                && !StatusCode::is_server_error(&status_code)
            {
                panic!("{}: Invalid HTTP error code", status_code);
            }

            let paths = paths.as_array().expect("Invalid error paths type");
            if paths.len() == 0 {
                return None;
            }
            Some(config_url_error(code, paths, path_prefix))
        })
        .collect::<HashMap<u16, Vec<String>>>();
    errors
}

fn config_url_error(code: &str, paths: &Vec<toml::Value>, path_prefix: &str) -> (u16, Vec<String>) {
    let full_paths = paths
        .into_iter()
        .map(|path| {
            let path = match path.as_str() {
                Some(x) => {
                    if x.len() == 0 {
                        panic!("{}: Empty error path", code);
                    }
                    x
                }
                _ => panic!("{}: Invalid error path", code),
            };
            let full_path = format!("/{}/{}/", path_prefix, path.to_owned()).replace("//", "/");
            let full_path_wo_trailing_slash = &full_path[..full_path.len() - 1];
            full_path_wo_trailing_slash.to_string()
        })
        .collect::<Vec<String>>();

    println!(
        "[error] {} = {}",
        style(code).magenta(),
        style(full_paths.join(", ")).yellow()
    );
    (
        code.parse::<u16>()
            .expect(&format!("{}: Invalid code", code)),
        full_paths.to_owned(),
    )
}

fn validate_paths(
    paths: &Option<HashMap<String, String>>,
    errors: &Option<HashMap<u16, Vec<String>>>,
) -> String {
    let url_paths = if let Some(x) = paths {
        x.into_iter()
            .map(|(y, _)| y.as_str())
            .collect::<Vec<&str>>()
    } else {
        Vec::<&str>::new()
    };
    let error_paths = if let Some(x) = errors {
        x.into_iter()
            .map(|(_, y)| y.into_iter().map(|z| z.as_str()).collect::<Vec<&str>>())
            .flatten()
            .collect::<Vec<&str>>()
    } else {
        Vec::<&str>::new()
    };
    let concatted = [url_paths, error_paths].concat();

    for i in 0..concatted.len() {
        for j in (i + 1)..concatted.len() {
            if concatted[i] == concatted[j] {
                return concatted[i].to_owned();
            }
        }
    }
    String::new()
}
