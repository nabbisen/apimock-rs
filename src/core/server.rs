use console::style;
use http_body_util::{BodyExt, Empty, Full};
use hyper::{
    body::{Bytes, Incoming},
    header::{
        HeaderValue, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
        ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE,
    },
    http::{request::Parts, response::Builder, Error},
    Request, Response, StatusCode,
};
use json5;
use serde_json::{json, Map, Value};
use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::sync::Mutex;
use tokio::time;

use crate::core::constant::server::CSV_RECORDS_DEFAULT_KEY;

use super::{app_state::AppState, server_middleware};
use super::{config::ConfigUrlPaths, util::jsonpath_value};
use super::{
    config::{Config, HeaderConfig, HeaderId, JsonpathMatchingPattern, PathConfig, VerboseConfig},
    types::BoxBody,
};

const DEFAULT_PLAIN_TEXT_CONTENT_TYPE: &str = "text/plain";
const JSON_EXTENSIONS: [&str; 2] = ["json", "json5"];

/// entry point of http requests handler service
pub async fn handle(
    req: Request<Incoming>,
    app_state: Arc<Mutex<AppState>>,
) -> Result<Response<BoxBody>, Error> {
    let (parts, body) = req.into_parts();
    let request_uri_path = uri_path(parts.uri.path());

    // todo: old code to remove:
    // let body_bytes = hyper::body::to_bytes(request_body).await.unwrap();
    // Bytes::copy_to_bytes(request_body).await.unwrap();
    let request_body_bytes = body
        .boxed()
        .collect()
        .await
        .expect("failed to collect request incoming body")
        .to_bytes();
    let request_body_json_value: Option<Value> = if 0 < request_body_bytes.len() {
        serde_json::from_slice(&request_body_bytes)
            .expect("failed to get json value from request body")
    } else {
        None
    };

    let shared_app_state = { app_state.lock().await.clone() };

    // middleware if activated
    if let Some(middleware) = shared_app_state.middleware {
        let middleware_response_filepath = server_middleware::handle(
            request_uri_path.as_str(),
            request_body_json_value.as_ref(),
            &middleware.engine,
            &middleware.ast,
        );
        if let Some(middleware_response_filepath) = middleware_response_filepath {
            log(
                request_uri_path.as_str(),
                &parts,
                request_body_json_value.as_ref(),
                shared_app_state.config.verbose,
            );

            return file_to_response(middleware_response_filepath.as_str(), None, None);
        }
    }

    // app handle driven by config
    let config = shared_app_state.config;

    // fixed response with `always` in config
    if let Some(x) = handle_always(config.always.as_ref()) {
        log(
            request_uri_path.as_str(),
            &parts,
            None,
            config.verbose.clone(),
        );

        response_wait(config.response_wait_millis).await;
        return Ok(x.unwrap());
    }

    // config update
    {
        let mut shared_app_state = { app_state.lock().await.clone() };
        let mut config = shared_app_state.config;

        if let Some(x) = handle_data_dir_query_path(&config, request_uri_path.as_str()) {
            let res = if !x.is_empty() {
                let old_data_dir = config.data_dir.clone().unwrap();
                let data_dir = x.strip_prefix("/").unwrap();

                config.data_dir = Some(data_dir.to_owned());
                config.update_paths(data_dir, old_data_dir.as_str());
                shared_app_state.config = config.clone();

                plain_text_response(data_dir, None)
            } else {
                plain_text_response(config.data_dir.clone().unwrap().as_str(), None)
            };

            log(
                request_uri_path.as_str(),
                &parts,
                None,
                config.verbose.clone(),
            );
            log::info!(" * [url.data_dir] updated.\n");
            config.print_paths();
            log::info!("");

            return res;
        }
    }

    log(
        request_uri_path.as_str(),
        &parts,
        request_body_json_value.as_ref(),
        config.verbose,
    );

    // wait to mimic real world server behavior
    response_wait(config.response_wait_millis).await;

    // response with static paths routing
    if let Some(paths) = &config.paths {
        if let Some(x) = handle_static_path(
            request_uri_path.as_str(),
            request_body_json_value.as_ref(),
            paths,
            config.paths_jsonpath_patterns.as_ref(),
            config.headers.as_ref(),
        )
        .await
        {
            return x;
        }
    }

    // response with dynamic paths routing
    if let Some(dyn_data_dir) = &config.dyn_data_dir.clone() {
        handle_dyn_path(request_uri_path.as_str(), dyn_data_dir.as_str())
    } else {
        not_found_response()
    }
}

/// print out logs
fn log(
    request_uri_path: &str,
    request_header: &Parts,
    request_body_json_value: Option<&Value>,
    verbose: VerboseConfig,
) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let hours = (now / 3600) % 24;
    let minutes = (now / 60) % 60;
    let seconds = now % 60;
    let timestamp = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

    // uri and timestamp (base)
    let mut printed = format!(
        "<- {} (request got at {} UTC)",
        style(request_uri_path).yellow(),
        timestamp
    );

    // headers
    if verbose.header || verbose.body {
        printed.push_str("\n");
    }
    if verbose.header {
        let headers = request_header
            .headers
            .iter()
            .map(|x| format!("\n{}: {}", x.0, x.1.to_str().unwrap()))
            .collect::<String>();
        let printed_headers = format!(
            "({:?}, {}){}",
            request_header.version, request_header.method, headers
        );
        printed = format!("{}{}", printed, style(printed_headers).magenta());
    }
    // body (json params)
    let mut is_verbose_body = false;
    if verbose.body {
        if let Some(request_body_json_value) = request_body_json_value {
            let body_str = request_body_json_value.to_string();
            printed = format!("{}\n{}", printed, style(body_str).green());
            is_verbose_body = true;
        }
    }
    if verbose.header || is_verbose_body {
        printed.push_str("\n");
    }

    log::info!("{}", printed);
}

/// sleep
async fn response_wait(millis: u64) {
    time::sleep(Duration::from_millis(millis)).await
}

/// handle on `always` config
fn handle_always(always: Option<&String>) -> Option<Result<Response<BoxBody>, Error>> {
    match always {
        Some(x) => {
            let response =
                json_response_base(None, None).body(Full::new(Bytes::from(x.to_owned())).boxed());
            Some(response)
        }
        _ => None,
    }
}

/// handle on `data_dir_query_path` config
fn handle_data_dir_query_path(config: &Config, request_uri_path: &str) -> Option<String> {
    if request_uri_path == "" || config.data_dir_query_path.is_none() {
        return None;
    }

    let data_dir_query_path = config.data_dir_query_path.clone().unwrap();

    let stripped = request_uri_path
        .strip_prefix("/")
        .unwrap()
        .strip_prefix(data_dir_query_path.as_str());
    match stripped {
        Some(x) => return Some(x.to_owned()),
        None => return None,
    }
}

/// format uri path
///
/// omit leading slash
fn uri_path(uri_path: &str) -> String {
    if uri_path.chars().filter(|&c| c == '/').count() == 1 {
        uri_path.to_owned()
    } else if uri_path.ends_with("/") {
        uri_path[..uri_path.len() - 1].to_owned()
    } else {
        uri_path.to_owned()
    }
}

/// handle on `data_dir` paths (static json responses)
async fn handle_static_path(
    request_uri_path: &str,
    request_body_json_value: Option<&Value>,
    path_configs: &ConfigUrlPaths,
    paths_jsonpath_patterns: Option<
        &HashMap<String, HashMap<String, Vec<JsonpathMatchingPattern>>>,
    >,
    headers: Option<&HashMap<HeaderId, HeaderConfig>>,
) -> Option<Result<Response<BoxBody>, Error>> {
    let path_config_hashmap = path_configs
        .iter()
        .find(|(key, _)| key.as_str() == request_uri_path);
    if let Some(path_config_hashmap) = path_config_hashmap {
        let mut path_config = path_config_hashmap.1.clone();

        if let Some(request_body_json_value) = request_body_json_value {
            match_jsonpath_patterns(
                &mut path_config,
                request_uri_path,
                request_body_json_value,
                paths_jsonpath_patterns,
            )
            .await;
        }

        response_wait(path_config.response_wait_more_millis).await;

        let response = Some(static_path_response(
            &path_config,
            headers,
            request_uri_path,
        ));
        return response;
    }

    None
}

/// update path config if matcher finds pair
async fn match_jsonpath_patterns(
    path_config: &mut PathConfig,
    request_uri_path: &str,
    request_body_json_value: &Value,
    paths_jsonpath_patterns: Option<
        &HashMap<String, HashMap<String, Vec<JsonpathMatchingPattern>>>,
    >,
) {
    if let Some(paths_jsonpath_patterns) = paths_jsonpath_patterns {
        if let Some(key) = paths_jsonpath_patterns
            .keys()
            .find(|x| x.as_str() == request_uri_path)
        {
            let jsonpath_patterns = paths_jsonpath_patterns.get(key).unwrap();
            for jsonpath in jsonpath_patterns.keys() {
                if let Some(value) = jsonpath_value(request_body_json_value, jsonpath) {
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

/// response on `data_dir` paths
fn static_path_response(
    path_config: &PathConfig,
    headers: Option<&HashMap<HeaderId, HeaderConfig>>,
    request_uri_path: &str,
) -> Result<Response<BoxBody>, Error> {
    if let Some(_) = &path_config.data_src {
        return static_path_data_src_reponse(path_config, headers, request_uri_path);
    }

    if let Some(data_text) = &path_config.data_text {
        return json_response_base(path_config.headers.as_ref(), headers)
            .status(path_config.code)
            .body(Full::new(Bytes::from(data_text.to_owned())).boxed());
    }

    response_base(path_config.headers.as_ref(), headers)
        .status(path_config.code)
        .body(Empty::new().boxed())
}

/// json file on `data_src` response on `data_dir` paths
fn static_path_data_src_reponse(
    path_config: &PathConfig,
    headers: Option<&HashMap<HeaderId, HeaderConfig>>,
    request_uri_path: &str,
) -> Result<Response<BoxBody>, Error> {
    match path_config.data_src.as_ref() {
        Some(data_src) => {
            file_to_response(data_src.as_str(), path_config.headers.as_ref(), headers)
        }
        None => internal_server_error_response(
            format!("{}: data_src is missing", request_uri_path).as_str(),
        ),
    }
}

/// handle on `dyn_data_dir` (dynamic json responses)
fn handle_dyn_path(request_uri_path: &str, dyn_data_dir: &str) -> Result<Response<BoxBody>, Error> {
    let request_path =
        Path::new(dyn_data_dir).join(request_uri_path.strip_prefix("/").unwrap_or_default());

    let dir = request_path.parent().unwrap();
    if !Path::new(dir).exists() {
        return not_found_response();
    }

    let request_file_name = request_path.file_name().expect("failed to get file name");

    let mut found = None;
    for entry in fs::read_dir(dir).unwrap() {
        let entry_path = entry.unwrap().path();

        if entry_path
            .file_name()
            .unwrap_or_default()
            .to_ascii_lowercase()
            == request_file_name.to_ascii_lowercase()
        {
            found = Some(entry_path.as_path().to_owned());
            break;
        }
    }
    if found.is_none() && file_is_json(request_path.as_path()) {
        for entry in fs::read_dir(dir).unwrap() {
            let entry_path = entry.unwrap().path();

            if is_equivalent_json_file(request_path.as_ref(), entry_path.as_ref()) {
                found = Some(entry_path.as_path().to_owned());
                break;
            }
        }
    }

    match found {
        Some(found) => {
            let filepath = found.to_str().unwrap_or_default();
            file_to_response(filepath, None, None)
        }
        None => not_found_response(),
    }
}

/// check if file is json
fn file_is_json(p: &Path) -> bool {
    JSON_EXTENSIONS.contains(
        &p.extension()
            .unwrap_or_default()
            .to_ascii_lowercase()
            .to_str()
            .unwrap_or_default(),
    )
}

/// check if two json files are equivalent to each
fn is_equivalent_json_file(request_path: &Path, entry_path: &Path) -> bool {
    let request_file_stem = request_path
        .file_stem()
        .expect("failed to get requestfile stem");
    let request_ext = request_path
        .extension()
        .unwrap_or_default()
        .to_ascii_lowercase();

    let entry_file_stem = entry_path
        .file_stem()
        .expect("failed to get entry file stem");
    let entry_ext = entry_path
        .extension()
        .unwrap_or_default()
        .to_ascii_lowercase();

    request_file_stem == entry_file_stem
        && JSON_EXTENSIONS.contains(&request_ext.to_str().expect("failed to get requestfile ext"))
        && JSON_EXTENSIONS.contains(&entry_ext.to_str().expect("failed to get entry file ext"))
}

/// response from file path
fn file_to_response(
    filepath: &str,
    path_headers: Option<&Vec<String>>,
    headers: Option<&HashMap<HeaderId, HeaderConfig>>,
) -> Result<Response<BoxBody>, Error> {
    if Path::new(filepath).is_dir() {
        return bad_request_response(format!("{} is directory", filepath).as_str());
    }

    match std::fs::read_to_string(filepath) {
        Ok(content) => text_file_to_response(content.as_str(), filepath, path_headers, headers),
        Err(_) => match std::fs::read(filepath) {
            Ok(content) => binary_file_to_response(content.as_ref()),
            Err(err) => internal_server_error_response(
                format!("{}: failed to read file - {}", filepath, err).as_str(),
            ),
        },
    }
}

/// text file response
fn text_file_to_response(
    content: &str,
    filepath: &str,
    path_headers: Option<&Vec<String>>,
    headers: Option<&HashMap<HeaderId, HeaderConfig>>,
) -> Result<Response<BoxBody>, Error> {
    match Path::new(filepath)
        .extension()
        .unwrap_or_default()
        .to_ascii_lowercase()
        .to_str()
    {
        Some(ext) => match ext {
            "json" | "json5" => {
                json_text_file_to_response(content, filepath, path_headers, headers)
            }
            "csv" => csv_text_file_to_response(content, filepath, path_headers, headers),
            _ => plain_text_response(content, Some(text_file_content_type(ext).as_str())),
        },
        None => plain_text_response(content, None),
    }
}

/// content type from text file extension
fn text_file_content_type(ext: &str) -> String {
    let ret = match ext {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        _ => DEFAULT_PLAIN_TEXT_CONTENT_TYPE,
    };
    ret.to_owned()
}

/// json file response
fn json_text_file_to_response(
    content: &str,
    filepath: &str,
    path_headers: Option<&Vec<String>>,
    headers: Option<&HashMap<HeaderId, HeaderConfig>>,
) -> Result<Response<BoxBody>, Error> {
    match json5::from_str::<Value>(content) {
        Ok(json) => {
            let body = json.to_string();
            json_response_base(path_headers, headers)
                .status(StatusCode::OK)
                .body(Full::new(Bytes::from(body)).boxed())
        }
        _ => internal_server_error_response(format!("{}: Invalid json content", filepath).as_str()),
    }
}

/// csv file response
fn csv_text_file_to_response(
    content: &str,
    filepath: &str,
    path_headers: Option<&Vec<String>>,
    headers: Option<&HashMap<HeaderId, HeaderConfig>>,
) -> Result<Response<BoxBody>, Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(content.as_bytes());

    let csv_headers = if let Ok(csv_headers) = rdr.headers() {
        csv_headers.clone()
    } else {
        return internal_server_error_response(
            format!("{}: failed to analyze csv headers", filepath).as_str(),
        );
    };

    let rows = rdr
        .records()
        .map(|result| {
            let record = result?;
            let obj = csv_headers
                .iter()
                .zip(record.iter())
                .map(|(k, v)| (k.to_string(), Value::String(v.to_string())))
                .collect::<Map<_, _>>();
            Ok(Value::Object(obj))
        })
        .collect::<Result<Vec<Value>, csv::Error>>();

    match rows {
        Ok(rows) => {
            let json_value = json!({ CSV_RECORDS_DEFAULT_KEY: &rows });
            let body = serde_json::to_string(&json_value);
            match body {
                Ok(body) => json_response_base(path_headers, headers)
                    .status(StatusCode::OK)
                    .body(Full::new(Bytes::from(body)).boxed()),
                Err(err) => internal_server_error_response(
                    format!(
                        "{}: failed to convert csv records to json response - {}",
                        filepath, err
                    )
                    .as_str(),
                ),
            }
        }
        Err(err) => internal_server_error_response(
            format!("{}: failed to analyze csv records - {}", filepath, err).as_str(),
        ),
    }
}

/// binary file response
fn binary_file_to_response(content: &Vec<u8>) -> Result<Response<BoxBody>, Error> {
    let content = content.to_owned();
    response_base(None, None)
        .status(StatusCode::OK)
        .header(
            CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        )
        .body(Full::new(Bytes::from(content)).boxed())
}

/// response base on any
fn response_base(
    path_headers: Option<&Vec<String>>,
    headers: Option<&HashMap<HeaderId, HeaderConfig>>,
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
    path_headers: Option<&Vec<String>>,
    headers: Option<&HashMap<HeaderId, HeaderConfig>>,
) -> Builder {
    response_base(path_headers, headers)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
}

/// plain text response
fn plain_text_response(
    content: &str,
    content_type: Option<&str>,
) -> Result<Response<BoxBody>, Error> {
    let default_content_type = HeaderValue::from_static(DEFAULT_PLAIN_TEXT_CONTENT_TYPE);
    let response_content_type = if let Some(content_type) = content_type {
        HeaderValue::from_str(content_type).unwrap_or_else(|_| default_content_type)
    } else {
        default_content_type
    };

    response_base(None, None)
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, response_content_type)
        .body(Full::new(Bytes::from(content.to_owned())).boxed())
}

/// error response on http BAD_REQUEST (400)
fn bad_request_response(msg: &str) -> Result<Response<BoxBody>, Error> {
    response_base(None, None)
        .status(StatusCode::BAD_REQUEST)
        .body(Full::new(Bytes::from(msg.to_owned())).boxed())
}

/// error response on http NOT_FOUND (404)
fn not_found_response() -> Result<Response<BoxBody>, Error> {
    response_base(None, None)
        .status(StatusCode::NOT_FOUND)
        .body(Empty::new().boxed())
}

/// error response on http INTERNAL_SERVER_ERROR (500)
fn internal_server_error_response(msg: &str) -> Result<Response<BoxBody>, Error> {
    response_base(None, None)
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Full::new(Bytes::from(msg.to_owned())).boxed())
}
