use hyper::header::{
    HeaderValue, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE,
};
use hyper::{http::response::Builder, Body, Request, Response, StatusCode};
use json5;
use serde_json::Value;
use std::collections::HashMap;
use std::convert::Infallible;
use std::fs;
use std::path::Path;

use crate::config::Config;

pub async fn handle(req: Request<Body>, config: Config) -> Result<Response<Body>, Infallible> {
    match handle_always(&config.always) {
        Some(x) => return x,
        _ => (),
    }

    let path = uri_path(req.uri().path());
    let url_error = handle_url_errors(path, &config.errors);
    match url_error {
        Some(x) => return x,
        _ => (),
    }
    let url_path = handle_url_paths(path, &config.paths.clone().unwrap());
    match url_path {
        Some(x) => return x,
        _ => (),
    }
    handle_dyn_paths(path, &config.dyn_data_dir.clone().unwrap().as_str())
        .expect("Unknown error occurred")
}

fn handle_always(always: &Option<String>) -> Option<Result<Response<Body>, Infallible>> {
    match always {
        Some(x) => {
            let response = json_response_base()
                .body(Body::from(x.to_owned()))
                .expect("Invalid always value");
            Some(Ok(response))
        }
        _ => None,
    }
}

fn uri_path(uri_path: &str) -> &str {
    if uri_path.ends_with("/") {
        &uri_path[..uri_path.len() - 1]
    } else {
        uri_path
    }
}

fn handle_url_errors(
    path: &str,
    errors: &Option<HashMap<u16, Vec<String>>>,
) -> Option<Result<Response<Body>, Infallible>> {
    if let Some(errors) = errors {
        for (code, paths) in errors {
            if paths.into_iter().any(|x| x.as_str() == path) {
                let response = Ok(response_base()
                    .status(code.to_owned())
                    .body(Body::empty())
                    .unwrap());
                return Some(response);
            }
        }
    }
    None
}

fn handle_url_paths(
    path: &str,
    paths: &HashMap<String, String>,
) -> Option<Result<Response<Body>, Infallible>> {
    let json_file = paths.get(path);
    let response = match json_file {
        Some(json_file) => match std::fs::read_to_string(json_file) {
            Ok(content) => match json5::from_str::<Value>(&content) {
                Ok(json) => {
                    let body = json.to_string();
                    Some(
                        json_response_base()
                            .status(StatusCode::OK)
                            .body(Body::from(body)),
                    )
                }
                _ => Some(
                    response_base()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Invalid json content")),
                ),
            },
            _ => Some(
                response_base()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from("Missing json file")),
            ),
        },
        _ => None,
    };

    match response {
        Some(Ok(response)) => Some(Ok(response)),
        _ => None,
    }
}

fn json_response_base() -> Builder {
    response_base().header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
}

fn response_base() -> Builder {
    Response::builder()
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"))
        .header(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"))
        .header(
            ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("GET, POST, OPTIONS"),
        )
}

fn handle_dyn_paths(path: &str, dyn_data_dir: &str) -> Option<Result<Response<Body>, Infallible>> {
    let p = Path::new(dyn_data_dir).join(path.strip_prefix("/").unwrap());
    let dir = p.parent().unwrap();
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
                    json_response_base()
                        .status(StatusCode::OK)
                        .body(Body::from(body))
                }
                _ => response_base()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from("Invalid json content")),
            },
            _ => response_base()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty()),
        },
        _ => response_base()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty()),
    };

    Some(Ok(response.unwrap()))
}
