use console::style;
use hyper::StatusCode;
use json5;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::net::SocketAddr;
use std::path::Path;
use toml;

pub type UrlPath = String;
pub type HeaderId = String;

/// app config
#[derive(Clone, Default)]
pub struct Config {
    pub port: u16,
    pub addr: Option<SocketAddr>,
    pub dyn_data_dir: Option<String>,
    pub always: Option<String>,
    pub path_prefix: Option<String>,
    pub data_dir: Option<String>,
    pub data_dir_query_path: Option<String>,
    pub headers: Option<HashMap<HeaderId, HeaderConfig>>,
    pub paths: Option<HashMap<UrlPath, PathConfig>>,
    config_path: Option<String>,
}

/// response content related to api uri path
#[derive(Clone, Default)]
pub struct PathConfig {
    pub code: StatusCode,
    pub headers: Option<Vec<String>>,
    pub data_src: Option<String>,
    pub data_text: Option<String>,
}

/// http headers on responses
#[derive(Clone, Default)]
pub struct HeaderConfig {
    pub key: String,
    pub value: Option<String>,
}

pub const DEFAULT_LISTEN_PORT: u16 = 3001;

const DEFAULT_DYN_DATA_DIR: &str = "apimock-data";
const CONFIG_SECTION_GENERAL: &str = "general";
const CONFIG_SECTION_URL: &str = "url";
const CONFIG_SECTION_URL_HEADERS: &str = "headers";
const CONFIG_SECTION_URL_PATHS: &str = "paths";
const CONFIG_SECTION_URL_RAW_PATH: &str = "raw_paths";
const ALWAYS_DEFAULT_MESSAGES: &str = "Hello, world from API Mock.\n(Responses can be modified with either config toml file or dynamic data directory.)";

/// app config
impl Config {
    /// new
    pub fn new(config_path: &str) -> Config {
        if !config_path.is_empty() && !Path::new(config_path).exists() {
            panic!("config file was specified but didn't exist: {}", config_path);
        }

        let mut config = Self::default_config();

        if config_path.is_empty() {
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
                println!(
                    "{}: config file is missing (config-less mode)\n",
                    config_path
                );
                config.print();
                return config;
            }
        }
        println!("[config] {}\n", config_path);

        config.config_path = Some(config_path.to_owned());

        let toml_string = fs::read_to_string(config_path).unwrap();
        let toml_content: toml::Value = toml::from_str(&toml_string)
            .expect(format!("{}: Invalid toml content", config_path).as_str());

        if let Some(general_config_content) = toml_content.get(CONFIG_SECTION_GENERAL) {
            config.general_config(&general_config_content);
        }
        if let Some(url_config_content) = toml_content.get(CONFIG_SECTION_URL) {
            config.url_config(&url_config_content);
        }

        config.validate();

        config.print();
        config
    }

    /// update `data_src` on static json responses when `data_dir` is updated
    pub fn update_paths(&mut self, data_dir: &str, old_data_dir: &str) {
        self.paths = Some(
            self.paths
                .clone()
                .unwrap()
                .into_iter()
                .map(|mut x| {
                    if let Some(data_src) = x.1.data_src {
                        let data_dir_wo_trailing_slash = data_dir.trim_end_matches('/');
                        let data_src_body =
                            if let Some(stripped) = data_src.strip_prefix(old_data_dir) {
                                stripped
                            } else {
                                data_src.as_str()
                            }
                            .trim_start_matches('/');
                        x.1.data_src =
                            Some(format!("{}/{}", data_dir_wo_trailing_slash, data_src_body));
                    }
                    x
                })
                .collect::<HashMap<String, PathConfig>>(),
        )
    }

    /// print out paths on `data_dir` (static json responses)
    pub fn print_paths(&self) {
        let paths = &self.paths.clone().unwrap();
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
    }

    /// wholly print out config
    fn print(&self) {
        if let Some(always) = &self.always {
            println!("[always] {}", always);
        }
        if let Some(data_dir) = &self.data_dir {
            println!("[data_dir] {}", data_dir);
        }
        if let Some(data_dir_query_path) = &self.data_dir_query_path {
            println!(
                "[data_dir_query_url] http://{}/{}",
                &self.addr.unwrap().to_string(),
                data_dir_query_path
            );
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
                self.print_paths();
                println!("------");
            }
        }
        if let Some(dyn_data_dir) = &self.dyn_data_dir {
            println!("[dyn_data_dir] {}", style(dyn_data_dir).green());
        }
    }

    /// app config default
    fn default_config() -> Config {
        let mut config = Config::default();
        config.port = DEFAULT_LISTEN_PORT;
        config.addr = Some(([127, 0, 0, 1], config.port).into());
        config
    }

    /// [general] section
    fn general_config(&mut self, general_config_content: &toml::Value) {
        let general_config = general_config_content.as_table().expect(
            format!(
                "{}: Invalid [{}] section",
                &self.config_path.clone().unwrap(),
                CONFIG_SECTION_GENERAL
            )
            .as_str(),
        );
        for (key, value) in general_config {
            match key.as_str() {
                "port" => match value.as_integer() {
                    Some(port) => {
                        self.port = port as u16;
                        self.addr = Some(([127, 0, 0, 1], self.port).into());
                    }
                    _ => (),
                },
                "dyn_data_dir" => match value.as_str() {
                    Some(dyn_data_dir) => self.dyn_data_dir = Some(dyn_data_dir.to_owned()),
                    _ => (),
                },
                "always" => match value.as_str() {
                    Some(always) => {
                        let _ = json5::from_str::<serde_json::Value>(&always).expect(
                            format!(
                                "{}: Invalid `always` value",
                                &self.config_path.clone().unwrap()
                            )
                            .as_str(),
                        );
                        self.always = Some(always.to_owned());
                        return;
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }

    /// [url] section
    fn url_config(&mut self, url_config_content: &toml::Value) {
        let url_config = url_config_content.as_table().expect(
            format!(
                "{}: Invalid [{}] section",
                &self.config_path.clone().unwrap(),
                CONFIG_SECTION_URL
            )
            .as_str(),
        );
        for (key, value) in url_config {
            match key.as_str() {
                "path_prefix" => match value.as_str() {
                    Some(path_prefix) => self.path_prefix = Some(path_prefix.to_owned()),
                    _ => (),
                },
                "data_dir" => match value.as_str() {
                    Some(data_dir) => self.data_dir = Some(data_dir.to_owned()),
                    _ => (),
                },
                "data_dir_query_path" => match value.as_str() {
                    Some(data_dir_query_path) => {
                        self.data_dir_query_path = Some(data_dir_query_path.to_owned())
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        if let Some(headers_config_content) = url_config.get(CONFIG_SECTION_URL_HEADERS) {
            self.config_url_headers(&headers_config_content);
        }
        if let Some(paths_config_content) = url_config.get(CONFIG_SECTION_URL_PATHS) {
            self.config_url_paths(&paths_config_content);
        }
        if let Some(raw_paths_config_content) = url_config.get(CONFIG_SECTION_URL_RAW_PATH) {
            self.config_url_raw_paths(&raw_paths_config_content);
        }
    }

    /// [url.headers] section
    fn config_url_headers(&mut self, headers_config_content: &toml::Value) {
        let table = headers_config_content.as_table().expect(
            format!(
                "{}: Invalid [{}] section",
                &self.config_path.clone().unwrap(),
                CONFIG_SECTION_URL_HEADERS
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

    /// [url.paths] section
    fn config_url_paths(&mut self, paths_config_content: &toml::Value) {
        self.paths = Some(self.url_paths(paths_config_content, false));
    }

    /// [url.raw_paths] section
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

    /// response defs related to api uri paths on static / dynamic json responses
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

    /// response def related to api uri path
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
                data_src: Some(self.data_src_path(file)),
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
                            ret.data_src = Some(self.data_src_path(file))
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

    /// `data_src` path on static json responses
    fn data_src_path(&self, file: &str) -> String {
        let data_dir = if let Some(x) = &self.data_dir.clone() {
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

    /// validate user settings in app config
    fn validate(&self) {
        if self.always.is_none() && (self.paths.is_none() || self.paths.clone().unwrap().len() == 0)
        {
            panic!("paths not defined");
        }

        if let Some(data_dir_query_path) = self.data_dir_query_path.clone() {
            if data_dir_query_path == "" {
                panic!("data_dir_query_path is set but empty");
            }
        }

        if let Some(paths) = &self.paths {
            for (path, path_config) in paths {
                if !path_config.data_src.is_none() && !path_config.data_text.is_none() {
                    panic!("can't define src and text on path: {}", path);
                }
            }
        }
    }
}
