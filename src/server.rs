use hyper::header::{
    HeaderValue, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE,
};
use hyper::{Body, Request, Response, StatusCode};
use json5;
use serde_json::Value;
use std::convert::Infallible;

use crate::config::Config;

pub async fn handle(req: Request<Body>, config: Config) -> Result<Response<Body>, Infallible> {
    match config.always {
        Some(always) => {
            let mut response = Response::new(Body::from(always));
            response
                .headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            return Ok(response);
        }
        _ => (),
    }

    let uri_path = req.uri().path();
    let path = if uri_path.ends_with("/") {
        &uri_path[..uri_path.len() - 1]
    } else {
        uri_path
    };

    let errors = config.errors.clone().unwrap();
    for (code, paths) in errors {
        if paths.into_iter().any(|x| x.as_str() == path) {
            return Ok(Response::builder()
                .status(code)
                .body(Body::empty())
                .unwrap());
        }
    }

    let paths = config.paths.clone().unwrap();
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
