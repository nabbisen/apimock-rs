use hyper::HeaderMap;
use tokio::task;

use std::{fs, path::Path};

use crate::core::{
    server::{
        response::{
            error_response::{internal_server_error_response, not_found_response},
            file_response::FileResponse,
        },
        types::BoxBody,
    },
    util::json::JSON_COMPATIBLE_EXTENSIONS,
};

/// handle on `fallback_respond_dir` (dynamic json responses)
pub async fn dyn_route_content(
    url_path: &str,
    fallback_respond_dir: &str,
    request_headers: &HeaderMap,
) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
    let request_path =
        Path::new(fallback_respond_dir).join(url_path.strip_prefix("/").unwrap_or_default());

    let request_file_name = request_path
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();

    let dir = match request_path.parent() {
        Some(dir) => {
            if dir.exists() {
                dir.to_owned()
            } else {
                return not_found_response(request_headers);
            }
        }
        None => {
            return internal_server_error_response(
                &format!("parent dir not found: url_path = {}", url_path),
                request_headers,
            )
        }
    };

    let dir_for_blocking_task = dir.clone();
    let entries = task::spawn_blocking(move || {
        let mut ret = Vec::new();
        let dir = dir_for_blocking_task.clone();
        match fs::read_dir(dir.as_path()) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => ret.push(entry),
                        Err(err) => {
                            let err_msg = format!(
                                "failed to get dir entry from dir: {} ({})",
                                dir.to_string_lossy(),
                                err
                            );
                            return Err(err_msg.clone());
                        }
                    }
                }
                Ok(ret)
            }
            Err(err) => {
                let err_msg = format!("failed to get dir: {} ({})", dir.to_string_lossy(), err);
                return Err(err_msg.clone());
            }
        }
    })
    .await;

    let mut found = None;

    let entries = match entries {
        Ok(entries) => match entries {
            Ok(entries) => entries,
            Err(err) => return internal_server_error_response(err.as_str(), request_headers),
        },
        Err(err) => {
            return internal_server_error_response(
                &format!(
                    "failed to get dir entries (async handling: {})",
                    err.to_string()
                ),
                request_headers,
            );
        }
    };

    for entry in entries {
        let entry_path = entry.path();

        if entry_path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string()
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
                let file_path = dir.join(format!("{}.{}", request_file_stem, ext));
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
            FileResponse::new(file_path, None, request_headers)
                .file_content_response()
                .await
        }
        None => not_found_response(request_headers),
    }
}
