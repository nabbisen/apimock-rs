use hyper::header::{
    HeaderValue, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE,
};
use hyper::{http::response::Builder, http::Error, Body, Request, Response, StatusCode};
use json5;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

use crate::config::{Config, HeaderConfig, HeaderId, PathConfig, UrlPath};

/// entry point of http requests handler service
pub async fn handle(
    req: Request<Body>,
    app_state: Arc<Mutex<Config>>,
) -> Result<Response<Body>, Error> {
    let mut config = app_state.lock().await;

    if let Some(x) = handle_always(&config.always) {
        return Ok(x.unwrap());
    }

    let path = uri_path(req.uri().path());

    if let Some(x) = handle_data_dir_query_path(&mut config, path) {
        return x;
    }

    log(path);

    if let Some(paths) = &config.paths {
        if let Some(x) = handle_static_path(path, paths, &config.headers) {
            return x;
        }
    }
    if let Some(dyn_data_dir) = &config.dyn_data_dir.clone() {
        handle_dyn_path(path, dyn_data_dir.as_str())
    } else {
        not_found_response()
    }
}

/// print out logs
fn log(path: &str) {
    println!("- request got: path = {path} (path)");
}

/// handle on `always` config
fn handle_always(always: &Option<String>) -> Option<Result<Response<Body>, Error>> {
    match always {
        Some(x) => {
            let response = json_response_base(&None, &None).body(Body::from(x.to_owned()));
            Some(response)
        }
        _ => None,
    }
}

/// handle on `data_dir_query_path` config
fn handle_data_dir_query_path(
    config: &mut MutexGuard<Config>,
    path: &str,
) -> Option<Result<Response<Body>, Error>> {
    if path == "" || config.data_dir_query_path.is_none() {
        return None;
    }

    let data_dir_query_path = config.data_dir_query_path.clone().unwrap();

    let stripped = path
        .strip_prefix("/")
        .unwrap()
        .strip_prefix(data_dir_query_path.as_str());
    match stripped {
        Some("") => {
            return Some(plain_text_response(
                config.data_dir.clone().unwrap().as_str(),
            ))
        }
        Some(stripped) => {
            let old_data_dir = config.data_dir.clone().unwrap();
            let data_dir = stripped.strip_prefix("/").unwrap();
            config.data_dir = Some(data_dir.to_owned());
            config.update_paths(data_dir, old_data_dir.as_str());
            config.print_paths();
            return Some(plain_text_response(data_dir));
        }
        None => return None,
    }
}

/// format uri path
/// 
/// omit leading slash
fn uri_path(uri_path: &str) -> &str {
    if uri_path.ends_with("/") {
        &uri_path[..uri_path.len() - 1]
    } else {
        uri_path
    }
}

/// handle on `data_dir` paths (static json responses)
fn handle_static_path(
    path: &str,
    path_configs: &HashMap<UrlPath, PathConfig>,
    headers: &Option<HashMap<HeaderId, HeaderConfig>>,
) -> Option<Result<Response<Body>, Error>> {
    let path_config_hashmap = path_configs.iter().find(|(key, _)| key.as_str() == path);
    let response = if let Some(path_config_hashmap) = path_config_hashmap {
        let path_config = path_config_hashmap.1;
        Some(static_path_response(&path_config, headers))
    } else {
        None
    };
    response
}

/// response on `data_dir` paths
fn static_path_response(
    path_config: &PathConfig,
    headers: &Option<HashMap<HeaderId, HeaderConfig>>,
) -> Result<Response<Body>, Error> {
    if let Some(_) = &path_config.data_src {
        return static_path_data_src_reponse(path_config, headers);
    }

    if let Some(data_text) = &path_config.data_text {
        return json_response_base(&path_config.headers, headers)
            .status(path_config.code)
            .body(Body::from(data_text.to_owned()));
    }

    response_base(&path_config.headers, headers)
        .status(path_config.code)
        .body(Body::from(""))
}

/// json file on `data_src` response on `data_dir` paths
fn static_path_data_src_reponse(
    path_config: &PathConfig,
    headers: &Option<HashMap<HeaderId, HeaderConfig>>,
) -> Result<Response<Body>, Error> {
    match std::fs::read_to_string(path_config.data_src.as_ref().unwrap()) {
        Ok(content) => match json5::from_str::<Value>(&content) {
            Ok(json) => {
                let json_text = json.to_string();
                json_response_base(&path_config.headers, headers)
                    .status(path_config.code)
                    .body(Body::from(json_text))
            }
            _ => bad_request_response("Invalid json content"),
        },
        _ => bad_request_response("Missing json file"),
    }
}

/// handle on `dyn_data_dir` (dynamic json responses)
fn handle_dyn_path(path: &str, dyn_data_dir: &str) -> Result<Response<Body>, Error> {
    let p = Path::new(dyn_data_dir).join(path.strip_prefix("/").unwrap_or_default());

    let dir = p.parent().unwrap();
    if !Path::new(dir).exists() {
        return not_found_response();
    }

    let file_name = p.file_stem().unwrap();

    let mut json_file = None;
    for entry in fs::read_dir(dir).unwrap() {
        let p = entry.unwrap().path();
        if p.file_stem().unwrap() != file_name {
            continue;
        }

        if p.file_name().unwrap().to_ascii_lowercase() == file_name.to_ascii_lowercase() {
            json_file = Some(p.as_path().to_owned());
            break;
        }
        let ext = p.extension().unwrap_or_default().to_ascii_lowercase();
        if ["json", "json5"].contains(&ext.to_str().unwrap()) {
            json_file = Some(p.as_path().to_owned());
            break;
        }
    }

    let response = match json_file {
        Some(json_file) => match std::fs::read_to_string(json_file) {
            Ok(content) => match json5::from_str::<Value>(&content) {
                Ok(json) => {
                    let body = json.to_string();
                    json_response_base(&None, &None)
                        .status(StatusCode::OK)
                        .body(Body::from(body))
                }
                _ => bad_request_response("Invalid json content"),
            },
            _ => not_found_response(),
        },
        _ => not_found_response(),
    };
    response
}

/// response base on any
fn response_base(
    path_headers: &Option<Vec<String>>,
    headers: &Option<HashMap<HeaderId, HeaderConfig>>,
) -> Builder {
    let mut ret = Response::builder()
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"))
        .header(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"))
        .header(
            ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("GET, POST, OPTIONS"),
        );
    if let Some(path_headers) = path_headers {
        let headers = headers.clone().unwrap();
        for path_header in path_headers {
            let header = headers.get(path_header).unwrap();
            ret = ret.header(header.key.as_str(), header.value.clone().unwrap().as_str());
        }
    }
    ret
}

/// response base on json response
fn json_response_base(
    path_headers: &Option<Vec<String>>,
    headers: &Option<HashMap<HeaderId, HeaderConfig>>,
) -> Builder {
    response_base(path_headers, headers)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
}

/// plain text response
fn plain_text_response(msg: &str) -> Result<Response<Body>, Error> {
    response_base(&None, &None)
        .status(StatusCode::OK)
        .body(Body::from(msg.to_owned()))
}

/// error response on http NOT_FOUND
fn not_found_response() -> Result<Response<Body>, Error> {
    response_base(&None, &None)
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
}

/// error response on http BAD_REQUEST
fn bad_request_response(msg: &str) -> Result<Response<Body>, Error> {
    response_base(&None, &None)
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from(msg.to_owned()))
}
