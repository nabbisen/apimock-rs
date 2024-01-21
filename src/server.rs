use std::convert::Infallible;
use hyper::{Body, Request, Response, StatusCode};
use hyper::header::{HeaderValue, CONTENT_TYPE};
use serde_json::Value;
use json5;

use crate::config::Config;

pub async fn handle(req: Request<Body>, config: Config) -> Result<Response<Body>, Infallible> {
    match config.always {
        Some(always) => {
            let mut response = Response::new(Body::from(always));
            response.headers_mut().insert(
                CONTENT_TYPE, 
                HeaderValue::from_static("application/json")
            );
            return Ok(response)
        },
        _ => ()
    }

    let path = req.uri().path();
    let path_wo_trailing_slash = if path.ends_with("/") { &path[..path.len() - 1] } else { path };

    let paths = config.paths.clone().unwrap();
    let json_file = paths.get(path_wo_trailing_slash);
    let body = match json_file {
        Some(json_file) => {
            match std::fs::read_to_string(json_file) {
                Ok(content) => {
                    match json5::from_str::<Value>(&content) {
                        Ok(json) => json.to_string(),
                        _ => return Ok(Response::builder().status(StatusCode::BAD_REQUEST).body(Body::from("Invalid json content")).unwrap())
                    }
                },
                _ => return Ok(Response::builder().status(StatusCode::BAD_REQUEST).body(Body::from("Missing json file")).unwrap())
            }
        },
        _ => return Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty()).unwrap())
    };
    
    let mut response = Response::new(Body::from(body));

    response.headers_mut().insert(
        CONTENT_TYPE, 
        HeaderValue::from_static("application/json")
    );

    Ok(response)
}
