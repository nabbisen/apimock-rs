
use std::{fs, path::Path};

use crate::core::server::{
    response::{error::not_found_response, file::FileResponse},
    types::BoxBody,
    util::{file_is_json, is_equivalent_json_file},
};

/// handle on `fallback_response_dir` (dynamic json responses)
pub fn dyn_route_content(
    uri_path: &str,
    fallback_response_dir: &str,
) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
    let request_path =
        Path::new(fallback_response_dir).join(uri_path.strip_prefix("/").unwrap_or_default());

    let dir = request_path.parent().unwrap();
    if !dir.exists() {
        return not_found_response();
    }

    let request_file_name = request_path.file_name().expect("failed to get file name");

    // todo: path /a/b/c -> res /a/b/c.json ?
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
            let file_path = found.to_str().unwrap_or_default();
            FileResponse::new(file_path, None).file_content_response()
        }
        None => not_found_response(),
    }
}
