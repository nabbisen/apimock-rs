use super::constant::config::*;
use console::style;
use hyper::http::StatusCode;
use json5;
use serde_json;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use toml;

pub type UrlPath = String;
pub type HeaderId = String;

/// app config
#[derive(Clone, Default)]
pub struct Config {
    // [general]
    pub ip_address: String,
    pub port: u16,
    pub dyn_data_dir: Option<String>,
    pub always: Option<String>,
    pub response_wait_millis: u64,
    pub verbose: VerboseConfig,
    // [url]
    pub path_prefix: Option<String>,
    pub data_dir: Option<String>,
    pub data_dir_query_path: Option<String>,
    pub headers: Option<HashMap<HeaderId, HeaderConfig>>,
    pub paths: Option<HashMap<UrlPath, PathConfig>>,
    pub paths_jsonpath_patterns:
        Option<HashMap<String, HashMap<String, Vec<JsonpathMatchingPattern>>>>,
    config_path: Option<String>,
}

/// verbose logs
#[derive(Clone, Default)]
pub struct VerboseConfig {
    pub header: bool,
    pub body: bool,
}

/// response content related to api uri path
#[derive(Clone, Default)]
pub struct PathConfig {
    pub code: StatusCode,
    pub headers: Option<Vec<String>>,
    pub data_src: Option<String>,
    pub data_text: Option<String>,
    pub response_wait_more_millis: u64,
}

/// http headers on responses
#[derive(Clone, Default)]
pub struct HeaderConfig {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Clone)]
pub struct JsonpathMatchingPattern {
    pub value: String,
    pub data_src: String,
}

/// app config
impl Config {
    /// create new instance
    pub fn new(config_path: &str) -> Config {
        if !config_path.is_empty() && !Path::new(config_path).exists() {
            panic!(
                "config file was specified but didn't exist: {}",
                config_path
            );
        }

        let exists_default_config = Path::new(CONFIG_FILENAME).exists();
        let config_path = if config_path.is_empty() && exists_default_config {
            CONFIG_FILENAME
        } else {
            config_path
        };

        let mut config = Self::default_config();

        if config_path.is_empty() && !exists_default_config {
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

    /// address listened to
    pub fn listen_address(&self) -> String {
        format!("{}:{}", self.ip_address, self.port)
    }

    /// update `data_src` on static json responses when `data_dir` is updated
    pub fn update_paths(&mut self, data_dir: &str, old_data_dir: &str) {
        // fn: get data_src with updated data_dir
        let updated_data_src = |data_src: &str, data_dir: &str| -> String {
            let data_dir_wo_trailing_slash = data_dir.trim_end_matches('/');
            let data_src_body = if let Some(stripped) = data_src.strip_prefix(old_data_dir) {
                stripped
            } else {
                data_src
            }
            .trim_start_matches('/');
            format!("{}/{}", data_dir_wo_trailing_slash, data_src_body)
        };

        // [url.paths] data_src
        let paths = Some(
            self.paths
                .clone()
                .unwrap()
                .into_iter()
                .map(|mut x| {
                    if let Some(data_src) = x.1.data_src {
                        x.1.data_src = Some(updated_data_src(data_src.as_str(), data_dir));
                    }
                    x
                })
                .collect::<HashMap<String, PathConfig>>(),
        );
        self.paths = paths;

        // [url.paths_patterns] data_src
        if let Some(paths_jsonpath_patterns) = self.paths_jsonpath_patterns.clone() {
            let mut updated_paths_jsonpath_patterns: HashMap<
                String,
                HashMap<String, Vec<JsonpathMatchingPattern>>,
            > = HashMap::new();
            paths_jsonpath_patterns.keys().for_each(|path| {
                let mut updated_jsonpath_patterns: HashMap<String, Vec<JsonpathMatchingPattern>> =
                    HashMap::new();
                let jsonpath_patterns = paths_jsonpath_patterns.get(path).unwrap();
                jsonpath_patterns.keys().for_each(|jsonpath| {
                    let patterns = jsonpath_patterns.get(jsonpath).unwrap();
                    let updated_patterns = patterns
                        .iter()
                        .map(|pattern| {
                            let mut updated_pattern = pattern.clone();
                            updated_pattern.data_src =
                                updated_data_src(pattern.data_src.as_str(), data_dir);
                            updated_pattern
                        })
                        .collect();
                    updated_jsonpath_patterns.insert(jsonpath.to_owned(), updated_patterns);
                });
                updated_paths_jsonpath_patterns.insert(path.to_owned(), updated_jsonpath_patterns);
            });
            self.paths_jsonpath_patterns = Some(updated_paths_jsonpath_patterns);
        }
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

            if let Some(path_jsonpath_patterns) = &self.paths_jsonpath_patterns {
                if let Some(jsonpath_patterns) = path_jsonpath_patterns.get(key) {
                    let mut keys: Vec<_> = jsonpath_patterns.keys().collect();
                    keys.sort();
                    println!(
                        " jsonpath {}",
                        keys.iter()
                            .map(|&jsonpath| {
                                jsonpath_patterns
                                    .get(jsonpath)
                                    .unwrap()
                                    .iter()
                                    .map(|pattern| {
                                        format!(
                                            "case {} = \"{}\"\n            => {}",
                                            style(jsonpath).yellow(),
                                            style(pattern.value.to_owned()).magenta(),
                                            style(pattern.data_src.to_owned()).green()
                                        )
                                    })
                                    .collect::<Vec<String>>()
                                    .join("\n          ")
                            })
                            .collect::<Vec<String>>()
                            .join("\n          ")
                    );
                }
            }
        }
    }

    /// wholly print out config
    fn print(&self) {
        if let Some(always) = &self.always {
            println!("[always] {}", always);
        }
        println!(
            "[response wait] {}",
            if 0 < self.response_wait_millis {
                format!("{} milliseconds", self.response_wait_millis)
            } else {
                "-".to_owned()
            }
        );
        println!(
            "[verbose] header = {}, body = {}",
            if self.verbose.header { "Yes" } else { "No" },
            if self.verbose.body { "Yes" } else { "No" }
        );
        println!("------");
        if let Some(data_dir) = &self.data_dir {
            println!("[data_dir] {}", data_dir);
        }
        if let Some(data_dir_query_path) = &self.data_dir_query_path {
            println!(
                "[data_dir_query_url] http://{}/{}",
                &self.listen_address(),
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
        config.ip_address = DEFAULT_LISTEN_IP_ADDRESS.to_owned();
        config.response_wait_millis = 0;
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
                "ip_address" => match value.as_str() {
                    Some(ip_address) => self.ip_address = ip_address.to_owned(),
                    _ => (),
                },
                "port" => match value.as_integer() {
                    Some(port) => {
                        self.port = port as u16;
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
                    }
                    _ => (),
                },
                "response_wait" => match value.as_integer() {
                    Some(response_wait_millis) => {
                        if response_wait_millis.is_negative() {
                            panic!("response_wait must be positive");
                        }
                        self.response_wait_millis = response_wait_millis.unsigned_abs();
                    }
                    _ => (),
                },
                "verbose" => {
                    let verbose = if let Some(t) = value.as_table() {
                        VerboseConfig {
                            header: if let Some(v) = t.get("header") {
                                v.as_bool().unwrap_or_default()
                            } else {
                                false
                            },
                            body: if let Some(v) = t.get("body") {
                                v.as_bool().unwrap_or_default()
                            } else {
                                false
                            },
                        }
                    } else if let Some(v) = value.as_bool() {
                        VerboseConfig { header: v, body: v }
                    } else {
                        VerboseConfig {
                            header: false,
                            body: false,
                        }
                    };
                    self.verbose = verbose;
                }
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
        if let Some(paths_jsonpath_patterns) =
            url_config.get(CONFIG_SECTION_URL_PATHS_JSONPATH_PATTERNS)
        {
            self.config_url_paths_jsonpath_patterns(&paths_jsonpath_patterns);
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

    /// [url.paths_patterns] section
    fn config_url_paths_jsonpath_patterns(&mut self, paths_jsonpath_patterns_json: &toml::Value) {
        let table = paths_jsonpath_patterns_json
            .as_table()
            .expect("`paths_jsonpath_patterns` should be table");

        let mut paths_jsonpath_patterns: HashMap<
            String,
            HashMap<String, Vec<JsonpathMatchingPattern>>,
        > = HashMap::new();
        for path in table.keys() {
            let value = table.get(path).expect(
                format!(
                    "paths_jsonpath_patterns: empty value is not allowed. key = {}",
                    path
                )
                .as_str(),
            );
            let table = value
                .as_table()
                .expect(format!("paths_jsonpath_patterns: must be table. key = {}", path).as_str());

            let mut jsonpath_patterns: HashMap<String, Vec<JsonpathMatchingPattern>> =
                HashMap::new();
            table.keys().for_each(|jsonpath| {
                let json = table.get(jsonpath).unwrap();
                let patterns_config = json.as_table().expect(
                    format!(
                        "paths_jsonpath_patterns: must be table as value to data_src. key = {}.{}",
                        path, jsonpath
                    )
                    .as_str(),
                );
                let patterns = patterns_config
                    .keys()
                    .map(|x| {
                        let mut chars = x.chars();
                        let first_char_in_value = chars.next().unwrap_or_default();
                        if first_char_in_value != '=' {
                            panic!(
                                "paths_jsonpath_patterns: must start with '='. key = {}.{}.{}",
                                path, jsonpath, x
                            );
                        }

                        let value: String = chars.collect();

                        let file = patterns_config.get(x).unwrap().as_str().expect(
                            format!(
                                "paths_jsonpath_patterns: data_src must be string. key = {}.{}.{}",
                                path, jsonpath, value
                            )
                            .as_str(),
                        );
                        let data_src = data_src_path(file, &self.data_dir);

                        JsonpathMatchingPattern {
                            value: value,
                            data_src: data_src,
                        }
                    })
                    .collect();

                jsonpath_patterns.insert(jsonpath.to_owned(), patterns);
            });

            if jsonpath_patterns.len() == 0 {
                continue;
            }

            let fullpath = fullpath(path, &self.path_prefix, false);
            paths_jsonpath_patterns.insert(fullpath, jsonpath_patterns);
        }

        self.paths_jsonpath_patterns = Some(paths_jsonpath_patterns);
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
        let full_path = fullpath(path, &self.path_prefix, is_raw_paths);

        let path_config = match path_config_content {
            toml::Value::String(file) => PathConfig {
                code: StatusCode::OK,
                headers: None,
                data_src: Some(data_src_path(file, &self.data_dir)),
                data_text: None,
                response_wait_more_millis: 0,
            },
            toml::Value::Table(table) => {
                let mut ret = PathConfig {
                    code: StatusCode::OK,
                    headers: None,
                    data_src: None,
                    data_text: None,
                    response_wait_more_millis: 0,
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
                            ret.data_src = Some(data_src_path(file, &self.data_dir))
                        }
                        "text" => ret.data_text = Some(value.as_str().unwrap().to_owned()),
                        "wait_more" => {
                            let response_wait_more_millis = value
                                .as_integer()
                                .expect(format!("{}: wait_more must be integer", value).as_str());
                            if response_wait_more_millis.is_negative() {
                                panic!("{}: wait_more must be positive", value);
                            }
                            ret.response_wait_more_millis =
                                response_wait_more_millis.unsigned_abs();
                        }
                        _ => panic!("{}", format!("{} is invalid", table).as_str()),
                    }
                }
                ret
            }
            _ => panic!("{}", format!("{} is invalid", path_config_content).as_str()),
        };

        (full_path, path_config)
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

/// api url full path
fn fullpath(path: &str, path_prefix: &Option<String>, is_raw_paths: bool) -> String {
    let possibly_w_trailing_slash = if is_raw_paths {
        format!("/{}/", path.to_string())
    } else {
        if let Some(path_prefix) = path_prefix {
            format!("/{}/{}/", path_prefix, path.to_string())
        } else {
            format!("/{}/", path.to_string())
        }
    }
    .replace("//", "/");

    (&possibly_w_trailing_slash[..possibly_w_trailing_slash.len() - 1]).to_owned()
}

/// `data_src` path on static json responses
fn data_src_path(file: &str, data_dir: &Option<String>) -> String {
    let data_dir = if let Some(x) = data_dir.clone() {
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

/// app config path
///
/// - if specified with command-line option, use it
/// - else use the default
pub fn config_path() -> String {
    let args: Vec<String> = env::args().collect();

    let config_option_entry = args
        .iter()
        .position(|arg| arg.as_str().eq("-c") || arg.as_str().eq("--config"));
    let config_path = match config_option_entry {
        Some(config_option_entry) => match args.get(config_option_entry + 1) {
            Some(config_option) => config_option,
            _ => "",
        },
        _ => "",
    };
    config_path.to_owned()
}
