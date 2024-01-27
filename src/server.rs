use hyper::header::{
    HeaderValue, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE,
};
use hyper::{Body, Request, Response, StatusCode};
use json5;
use serde_json::Value;
use std::collections::HashMap;
use std::convert::Infallible;

use crate::config::Config;

pub async fn handle(req: Request<Body>, config: Config) -> Result<Response<Body>, Infallible> {
    match handle_always(&config.always) {
        Some(x) => return x,
        _ => (),
    }

    let path = uri_path(req.uri().path());
    match handle_errors(path, &config.errors) {
        Some(x) => return x,
        _ => (),
    }
    handle_paths(path, &config.paths.unwrap())
}

fn handle_always(always: &Option<String>) -> Option<Result<Response<Body>, Infallible>> {
    match always {
        Some(x) => {
            let mut response = Response::new(Body::from(x.to_owned()));
            response
                .headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
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

fn handle_errors(
    path: &str,
    errors: &Option<HashMap<u16, Vec<String>>>,
) -> Option<Result<Response<Body>, Infallible>> {
    if let Some(errors) = errors {
        for (code, paths) in errors {
            if paths.into_iter().any(|x| x.as_str() == path) {
                let response = Ok(Response::builder()
                    .status(code.to_owned())
                    .body(Body::empty())
                    .unwrap());
                return Some(response);
            }
        }
    }
    None
}

fn handle_paths(path: &str, paths: &HashMap<String, String>) -> Result<Response<Body>, Infallible> {
    let json_file = paths.get(path);
    let body = match json_file {
        Some(json_file) => match std::fs::read_to_string(json_file) {
            Ok(content) => match json5::from_str::<Value>(&content) {
                Ok(json) => json.to_string(),
                _ => {
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Invalid json content"))
                        .unwrap())
                }
            },
            _ => {
                return Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from("Missing json file"))
                    .unwrap())
            }
        },
        _ => {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap())
        }
    };

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"))
        .header(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"))
        .header(
            ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("GET, POST, OPTIONS"),
        )
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .body(Body::from(body))
        .unwrap();
    Ok(response)
}
