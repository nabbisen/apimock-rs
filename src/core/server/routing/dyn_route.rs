use hyper::HeaderMap;

use std::{fs, path::Path};

use crate::core::{
    server::{
        response::{error_response::not_found_response, file_response::FileResponse},
        types::BoxBody,
    },
    util::json::JSON_COMPATIBLE_EXTENSIONS,
};

/// handle on `fallback_respond_dir` (dynamic json responses)
pub fn dyn_route_content(
    url_path: &str,
    fallback_respond_dir: &str,
    request_headers: &HeaderMap,
) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
    let request_path =
        Path::new(fallback_respond_dir).join(url_path.strip_prefix("/").unwrap_or_default());

    let dir = request_path.parent().unwrap();
    if !dir.exists() {
        return not_found_response(request_headers);
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

    if found.is_none() && request_path.extension().is_none() {
        let request_file_stem = match request_path.file_stem() {
            Some(file_stem_os_str) => match file_stem_os_str.to_str() {
                Some(file_stem) => Some(file_stem),
                None => None,
            },
            None => None,
        };
        if let Some(request_file_stem) = request_file_stem {
            for ext in JSON_COMPATIBLE_EXTENSIONS {
                let file_path = Path::new(dir).join(format!("{}.{}", request_file_stem, ext));
                if file_path.exists() {
                    found = Some(file_path.as_path().to_owned());
                    break;
                }
            }
        }
    }

    match found {
        Some(found) => {
            let file_path = found.to_str().unwrap_or_default();
            FileResponse::new(file_path, None, request_headers).file_content_response()
        }
        None => not_found_response(request_headers),
    }
}
