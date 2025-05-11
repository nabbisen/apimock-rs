use http_body_util::{BodyExt, Full};
use hyper::{
    body::{Bytes, Incoming},
    http::Error,
    Request, Response,
};
use jsonpath_pattern::match_jsonpath_patterns;
use response::{
    file_to_response, json_response_base, not_found_response, plain_text_response,
    static_path_response,
};
use serde_json::Value;
use tokio::sync::Mutex;
use tokio::time;

use std::{collections::HashMap, fs, path::Path, sync::Arc, time::Duration};

mod constant;
mod jsonpath_pattern;
mod request;
mod response;
mod server_middleware;
mod types;
mod util;

use crate::core::app::app_state::AppState;
use crate::core::{
    config::ConfigUrlPaths,
    config::{Config, HeaderConfig, HeaderId, JsonpathMatchingPattern},
};
use types::BoxBody;
use util::{file_is_json, is_equivalent_json_file};

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
            request::capture_in_log(
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
        request::capture_in_log(
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

            request::capture_in_log(
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

    request::capture_in_log(
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
