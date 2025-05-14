use http_body_util::{BodyExt, Full};
use hyper::{
    body::Bytes,
    header::{HeaderValue, CONTENT_TYPE},
    http::Error,
    StatusCode,
};

use std::{fs, path::Path};

use crate::core::server::{
    types::BoxBody,
    util::{file_is_json, is_equivalent_json_file},
};

use super::{error::not_found, file::file_content};

/// handle on `default_response_dir` (dynamic json responses)
pub fn dyn_route_content(
    uri_path: &str,
    default_response_dir: &str,
) -> Result<hyper::Response<BoxBody>, Error> {
    let request_path =
        Path::new(default_response_dir).join(uri_path.strip_prefix("/").unwrap_or_default());

    let dir = request_path.parent().unwrap();
    if !Path::new(dir).exists() {
        return not_found();
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
            file_content(filepath)
        }
        None => not_found(),
    }
}
