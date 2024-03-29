use console::style;
use hyper::StatusCode;
use json5;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use toml;

use crate::LISTEN_PORT;

pub type UrlPath = String;
pub type HeaderId = String;

#[derive(Clone, Default)]
pub struct Config {
    pub port: u16,
    pub dyn_data_dir: Option<String>,
    pub always: Option<String>,
    pub path_prefix: Option<String>,
    pub path_data_dir: Option<String>,
    pub headers: Option<HashMap<HeaderId, HeaderConfig>>,
    pub paths: Option<HashMap<UrlPath, PathConfig>>,
}
#[derive(Clone, Default)]
pub struct PathConfig {
    pub code: StatusCode,
    pub headers: Option<Vec<String>>,
    pub data_src: Option<String>,
    pub data_text: Option<String>,
}
#[derive(Clone, Default)]
pub struct HeaderConfig {
    pub key: String,
    pub value: Option<String>,
}

const DEFAULT_DYN_DATA_DIR: &str = "apimock-data";
const CONFIG_SECTION_GENERAL: &str = "general";
const CONFIG_SECTION_URL: &str = "url";
const CONFIG_SECTION_URL_HEADERS: &str = "headers";
const CONFIG_SECTION_URL_PATHS: &str = "paths";
const CONFIG_SECTION_URL_RAW_PATH: &str = "raw_paths";
const ALWAYS_DEFAULT_MESSAGES: &str = "Hello, world from API Mock.\n(Responses can be modified with either config toml file or dynamic data directory.)";

impl Config {
    pub fn new(config_path: &str) -> Config {
        let mut config = Self::default_config();

        if !Path::new(config_path).exists() {
            if !Path::new(DEFAULT_DYN_DATA_DIR).exists() {
                config.always = Some(ALWAYS_DEFAULT_MESSAGES.to_owned());
                println!(
                    "Both `{}` file and `{}/` directory are missing\n`always` option is activated\n",
                    config_path, DEFAULT_DYN_DATA_DIR
                );
                config.print();
                return config;
            } else {
                config.dyn_data_dir = Some(DEFAULT_DYN_DATA_DIR.to_owned());
                println!("{}: config file is missing (config-less mode)\n", config_path);
                config.print();
                return config;
            }
        }
        println!("[config] {}\n", config_path);

        let toml_string = fs::read_to_string(config_path).unwrap();
        let toml_content: toml::Value = toml::from_str(&toml_string)
            .expect(format!("{}: Invalid toml content", config_path).as_str());

        if let Some(general_config_content) = toml_content.get(CONFIG_SECTION_GENERAL) {
            config.general_config(&general_config_content, config_path);
        }
        if let Some(url_config_content) = toml_content.get(CONFIG_SECTION_URL) {
            config.url_config(&url_config_content, config_path);
        }

        config.validate();

        config.print();
        config
    }

    fn default_config() -> Config {
        let mut config = Config::default();
        config.port = LISTEN_PORT;
        config
    }

    fn general_config(&mut self, general_config_content: &toml::Value, config_path: &str) {
        let general_config = general_config_content.as_table().expect(
            format!(
                "{}: Invalid [{}] section",
                config_path, CONFIG_SECTION_GENERAL
            )
            .as_str(),
        );
        for (key, value) in general_config {
            match key.as_str() {
                "port" => match value.as_integer() {
                    Some(port) => self.port = port as u16,
                    _ => (),
                },
                "dyn_data_dir" => match value.as_str() {
                    Some(dyn_data_dir) => self.dyn_data_dir = Some(dyn_data_dir.to_owned()),
                    _ => (),
                },
                "always" => match value.as_str() {
                    Some(always) => {
                        let _ = json5::from_str::<serde_json::Value>(&always)
                            .expect(format!("{}: Invalid `always` value", config_path).as_str());
                        self.always = Some(always.to_owned());
                        return;
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }

    fn url_config(&mut self, url_config_content: &toml::Value, config_path: &str) {
        let url_config = url_config_content
            .as_table()
            .expect(format!("{}: Invalid [{}] section", config_path, CONFIG_SECTION_URL).as_str());
        for (key, value) in url_config {
            match key.as_str() {
                "path_prefix" => match value.as_str() {
                    Some(path_prefix) => self.path_prefix = Some(path_prefix.to_owned()),
                    _ => (),
                },
                "data_dir" => match value.as_str() {
                    Some(path_data_dir) => self.path_data_dir = Some(path_data_dir.to_owned()),
                    _ => (),
                },
                _ => (),
            }
        }

        if let Some(headers_config_content) = url_config.get(CONFIG_SECTION_URL_HEADERS) {
            self.config_url_headers(&headers_config_content, config_path);
        }
        if let Some(paths_config_content) = url_config.get(CONFIG_SECTION_URL_PATHS) {
            self.config_url_paths(&paths_config_content);
        }
        if let Some(raw_paths_config_content) = url_config.get(CONFIG_SECTION_URL_RAW_PATH) {
            self.config_url_raw_paths(&raw_paths_config_content);
        }
    }

    fn config_url_headers(&mut self, headers_config_content: &toml::Value, config_path: &str) {
        let table = headers_config_content.as_table().expect(
            format!(
                "{}: Invalid [{}] section",
                config_path, CONFIG_SECTION_URL_HEADERS
            )
            .as_str(),
        );
        let ret = table
            .iter()
            .map(|(id, key_value)| {
                let value = if let Some(x) = key_value.get("value") {
                    Some(x.as_str().unwrap().to_owned())
                } else {
                    None
                };
                let header_config = HeaderConfig {
                    key: key_value.get("key").unwrap().as_str().unwrap().to_owned(),
                    value: value,
                };
                (id.to_owned(), header_config)
            })
            .collect::<HashMap<HeaderId, HeaderConfig>>();
        self.headers = Some(HashMap::from(ret));
    }

    fn config_url_paths(&mut self, paths_config_content: &toml::Value) {
        self.paths = Some(self.url_paths(paths_config_content, false));
    }

    fn config_url_raw_paths(&mut self, raw_paths_config_content: &toml::Value) {
        let mut merged = if let Some(ref paths) = self.paths {
            paths.clone()
        } else {
            HashMap::new()
        };
        let raw_paths = self.url_paths(raw_paths_config_content, true);
        merged.extend(raw_paths);
        self.paths = Some(merged);
    }

    fn url_paths(
        &self,
        paths_config_content: &toml::Value,
        is_raw_paths: bool,
    ) -> HashMap<UrlPath, PathConfig> {
        let mut ret = HashMap::<UrlPath, PathConfig>::new();
        let p = paths_config_content.as_table().expect(
            format!(
                "[{}] Invalid entries",
                if is_raw_paths { "raw_paths" } else { "paths" }
            )
            .as_str(),
        );
        for (path, path_config_content) in p {
            let path_config = self.url_path(path, path_config_content, is_raw_paths);
            ret.insert(path_config.0, path_config.1);
        }
        ret
    }

    fn url_path(
        &self,
        path: &str,
        path_config_content: &toml::Value,
        is_raw_paths: bool,
    ) -> (UrlPath, PathConfig) {
        let full_path = {
            let possibly_w_trailing_slash = if is_raw_paths {
                format!("/{}/", path.to_string())
            } else {
                format!(
                    "/{}/{}/",
                    self.path_prefix.clone().unwrap_or_default(),
                    path.to_string()
                )
            }
            .replace("//", "/");
            (&possibly_w_trailing_slash[..possibly_w_trailing_slash.len() - 1]).to_owned()
        };

        let path_config = match path_config_content {
            toml::Value::String(file) => PathConfig {
                code: StatusCode::OK,
                headers: None,
                data_src: Some(self.data_path(file)),
                data_text: None,
            },
            toml::Value::Table(table) => {
                let mut ret = PathConfig {
                    code: StatusCode::OK,
                    headers: None,
                    data_src: None,
                    data_text: None,
                };
                for (key, value) in table {
                    match key.as_str() {
                        "code" => {
                            ret.code =
                                StatusCode::from_u16(value.as_integer().unwrap() as u16).unwrap()
                        }
                        "headers" => {
                            let array = value
                                .as_array()
                                .expect(format!("`{}` should be array", value).as_str());
                            if 0 < array.len() {
                                ret.headers = Some(
                                    array
                                        .iter()
                                        .map(|x| x.as_str().unwrap().to_string())
                                        .collect::<Vec<String>>(),
                                );
                                for header in ret.headers.clone().unwrap() {
                                    if self.headers.is_none()
                                        || !self
                                            .headers
                                            .clone()
                                            .unwrap()
                                            .keys()
                                            .any(|x| x == header.as_str())
                                    {
                                        panic!("{} is not found", header);
                                    }
                                }
                            }
                        }
                        "src" => {
                            let file = value.as_str().unwrap();
                            ret.data_src = Some(self.data_path(file))
                        }
                        "text" => ret.data_text = Some(value.as_str().unwrap().to_owned()),
                        _ => panic!("{}", format!("{} is invalid", table).as_str()),
                    }
                }
                ret
            }
            _ => panic!("{}", format!("{} is invalid", path_config_content).as_str()),
        };

        (full_path, path_config)
    }

    fn data_path(&self, file: &str) -> String {
        let data_dir = if let Some(x) = &self.path_data_dir.clone() {
            x.to_owned()
        } else {
            String::new()
        };
        let path = Path::new(data_dir.as_str())
            .join(file)
            .display()
            .to_string();
        let _ = fs::metadata(&path).expect(format!("`{}` is missing", path).as_str());
        path
    }

    fn validate(&self) {
        if self.always.is_none() && (self.paths.is_none() || self.paths.clone().unwrap().len() == 0)
        {
            panic!("paths not defined");
        }

        if let Some(paths) = &self.paths {
            for (path, path_config) in paths {
                if !path_config.data_src.is_none() && !path_config.data_text.is_none() {
                    panic!("can't define src and text on path: {}", path);
                }
            }
        }
    }

    fn print(&self) {
        if let Some(always) = &self.always {
            println!("[always] {}", always);
        }
        if let Some(path_data_dir) = &self.path_data_dir {
            println!("[path_data_dir] {}", path_data_dir);
        }
        if let Some(path_prefix) = &self.path_prefix {
            println!("[path_prefix] {}", path_prefix);
        }
        if let Some(headers) = &self.headers {
            if 0 < headers.len() {
                println!("------");
                let mut keys: Vec<_> = headers.keys().collect();
                keys.sort();
                for key in keys {
                    println!(
                        "[header] {} = {}{}",
                        style(headers.get_key_value(key).unwrap().0).magenta(),
                        headers.get(key).unwrap().key.clone(),
                        if let Some(value) = headers.get(key).unwrap().value.clone() {
                            format!(": {}", value)
                        } else {
                            String::new()
                        }
                    );
                }
            }
        }
        if let Some(paths) = &self.paths {
            if 0 < paths.len() {
                println!("------");
                let mut keys: Vec<_> = paths.keys().collect();
                keys.sort();
                for key in keys {
                    println!(
                        "[path] {} => [{}]{}{}",
                        style(paths.get_key_value(key).unwrap().0).yellow(),
                        paths.get(key).unwrap().code.as_u16(),
                        if let Some(data_src) = &paths.get(key).unwrap().data_src {
                            style(format!(" {}", data_src.as_str())).green()
                        } else {
                            if let Some(_) = &paths.get(key).unwrap().data_text {
                                style(" (text)".to_owned()).green()
                            } else {
                                style(String::new()).green()
                            }
                        },
                        if let Some(headers) = &paths.get(key).unwrap().headers {
                            let printed_outs = headers
                                .iter()
                                .map(|x| format!("{}", style(x.to_owned()).magenta()))
                                .collect::<Vec<String>>()
                                .join(", ");
                            format!(" {{{}}}", printed_outs)
                        } else {
                            String::new()
                        },
                    );
                }
                println!("------");
            }
        }
        if let Some(dyn_data_dir) = &self.dyn_data_dir {
            println!("[dyn_data_dir] {}", style(dyn_data_dir).green());
        }
    }
}
