use console::style;
use http_body_util::{BodyExt, Empty, Full};
use hyper::{
    body::{Bytes, Incoming},
    header::{
        HeaderValue, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
        ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE,
    },
    http::{response::Builder, Error},
    Request, Response, StatusCode,
};
use json5;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{collections::HashMap, convert::Infallible};
use tokio::sync::{Mutex, MutexGuard};
use tokio::time;

use crate::config::{Config, HeaderConfig, HeaderId, JsonpathMatchingPattern, PathConfig, UrlPath};
use crate::util::jsonpath_value;

type BoxBody = http_body_util::combinators::BoxBody<Bytes, Infallible>;

/// entry point of http requests handler service
pub async fn handle(
    req: Request<Incoming>,
    app_state: Arc<Mutex<Config>>,
) -> Result<Response<BoxBody>, Error> {
    let config = { app_state.lock().await.clone() };

    if let Some(x) = handle_always(&config.always) {
        response_wait(config.response_wait_millis).await;
        return Ok(x.unwrap());
    }

    let (parts, body) = req.into_parts();

    let path = uri_path(parts.uri.path());

    {
        let mut config = { app_state.lock().await };
        if let Some(x) = handle_data_dir_query_path(&mut config, path) {
            return x;
        }
    }

    log(path);

    response_wait(config.response_wait_millis).await;

    if let Some(paths) = &config.paths {
        if let Some(x) = handle_static_path(
            path,
            body,
            paths,
            &config.paths_jsonpath_patterns,
            &config.headers,
        )
        .await
        {
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
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let hours = (now / 3600) % 24;
    let minutes = (now / 60) % 60;
    let seconds = now % 60;
    let timestamp = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

    println!(
        " <- {} (request got at {} UTC)",
        style(path).yellow(),
        timestamp
    );
}

/// sleep
async fn response_wait(millis: u64) {
    time::sleep(Duration::from_millis(millis)).await
}

/// handle on `always` config
fn handle_always(always: &Option<String>) -> Option<Result<Response<BoxBody>, Error>> {
    match always {
        Some(x) => {
            let response =
                json_response_base(&None, &None).body(Full::new(Bytes::from(x.to_owned())).boxed());
            Some(response)
        }
        _ => None,
    }
}

/// handle on `data_dir_query_path` config
fn handle_data_dir_query_path(
    config: &mut MutexGuard<Config>,
    path: &str,
) -> Option<Result<Response<BoxBody>, Error>> {
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
    if uri_path.chars().filter(|&c| c == '/').count() == 1 {
        uri_path
    } else if uri_path.ends_with("/") {
        &uri_path[..uri_path.len() - 1]
    } else {
        uri_path
    }
}

/// handle on `data_dir` paths (static json responses)
async fn handle_static_path(
    path: &str,
    request_body: Incoming,
    path_configs: &HashMap<UrlPath, PathConfig>,
    paths_jsonpath_patterns: &Option<
        HashMap<String, HashMap<String, Vec<JsonpathMatchingPattern>>>,
    >,
    headers: &Option<HashMap<HeaderId, HeaderConfig>>,
) -> Option<Result<Response<BoxBody>, Error>> {
    let path_config_hashmap = path_configs.iter().find(|(key, _)| key.as_str() == path);
    if let Some(path_config_hashmap) = path_config_hashmap {
        let mut path_config = path_config_hashmap.1.clone();
        match_jsonpath_patterns(
            &mut path_config,
            path,
            request_body,
            paths_jsonpath_patterns,
        )
        .await;

        response_wait(path_config.response_wait_more_millis).await;

        let response = Some(static_path_response(&path_config, headers));
        return response;
    }

    None
}

/// update path config if matcher finds pair
async fn match_jsonpath_patterns(
    path_config: &mut PathConfig,
    path: &str,
    request_body: Incoming,
    paths_jsonpath_patterns: &Option<
        HashMap<String, HashMap<String, Vec<JsonpathMatchingPattern>>>,
    >,
) {
    if let Some(paths_jsonpath_patterns) = paths_jsonpath_patterns {
        if let Some(key) = paths_jsonpath_patterns.keys().find(|x| x.as_str() == path) {
            // let body_bytes = hyper::body::to_bytes(request_body).await.unwrap();
            // Bytes::copy_to_bytes(request_body).await.unwrap();
            let body_bytes = request_body
                .boxed()
                .collect()
                .await
                .expect("failed to collect request incoming body")
                .to_bytes();
            if 0 < body_bytes.len() {
                let json_value: Value = serde_json::from_slice(&body_bytes)
                    .expect("failed to get json value from request body");

                let jsonpath_patterns = paths_jsonpath_patterns.get(key).unwrap();
                for jsonpath in jsonpath_patterns.keys() {
                    if let Some(value) = jsonpath_value(&json_value, jsonpath) {
                        let request_json_value = match value {
                            Value::String(x) => x,
                            Value::Number(x) => x.to_string(),
                            _ => {
                                continue;
                            }
                        };
                        let patterns = jsonpath_patterns.get(jsonpath).unwrap();
                        if let Some(matched) = patterns
                            .iter()
                            .find(|x| x.value.as_str() == request_json_value.as_str())
                        {
                            // first matched only
                            path_config.data_src = Some(matched.data_src.to_owned());
                            break;
                        }
                    }
                }
            }
        }
    }
}

/// response on `data_dir` paths
fn static_path_response(
    path_config: &PathConfig,
    headers: &Option<HashMap<HeaderId, HeaderConfig>>,
) -> Result<Response<BoxBody>, Error> {
    if let Some(_) = &path_config.data_src {
        return static_path_data_src_reponse(path_config, headers);
    }

    if let Some(data_text) = &path_config.data_text {
        return json_response_base(&path_config.headers, headers)
            .status(path_config.code)
            .body(Full::new(Bytes::from(data_text.to_owned())).boxed());
    }

    response_base(&path_config.headers, headers)
        .status(path_config.code)
        .body(Empty::new().boxed())
}

/// json file on `data_src` response on `data_dir` paths
fn static_path_data_src_reponse(
    path_config: &PathConfig,
    headers: &Option<HashMap<HeaderId, HeaderConfig>>,
) -> Result<Response<BoxBody>, Error> {
    match std::fs::read_to_string(path_config.data_src.as_ref().unwrap()) {
        Ok(content) => match json5::from_str::<Value>(&content) {
            Ok(json) => {
                let json_text = json.to_string();
                json_response_base(&path_config.headers, headers)
                    .status(path_config.code)
                    .body(Full::new(Bytes::from(json_text)).boxed())
            }
            _ => bad_request_response("Invalid json content"),
        },
        _ => bad_request_response("Missing json file"),
    }
}

/// handle on `dyn_data_dir` (dynamic json responses)
fn handle_dyn_path(path: &str, dyn_data_dir: &str) -> Result<Response<BoxBody>, Error> {
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
                        .body(Full::new(Bytes::from(body)).boxed())
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
fn plain_text_response(msg: &str) -> Result<Response<BoxBody>, Error> {
    response_base(&None, &None)
        .status(StatusCode::OK)
        .body(Full::new(Bytes::from(msg.to_owned())).boxed())
}

/// error response on http NOT_FOUND
fn not_found_response() -> Result<Response<BoxBody>, Error> {
    response_base(&None, &None)
        .status(StatusCode::NOT_FOUND)
        .body(Empty::new().boxed()) // todo ? to Empty<Bytes>
}

/// error response on http BAD_REQUEST
fn bad_request_response(msg: &str) -> Result<Response<BoxBody>, Error> {
    response_base(&None, &None)
        .status(StatusCode::BAD_REQUEST)
        .body(Full::new(Bytes::from(msg.to_owned())).boxed())
}
