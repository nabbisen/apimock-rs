use hyper::header::HeaderValue;
use tokio::time;

use std::path::Path;
use std::time::Duration;

use super::constant::JSON_COMPATIBLE_EXTENSIONS;

/// check if content-type is application/json
/// supporting case when "application/json; charset=utf-8"
pub fn content_type_is_application_json(content_type: &HeaderValue) -> bool {
    if let Ok(content_type) = content_type.to_str() {
        content_type
            .trim_start()
            .to_ascii_lowercase()
            .starts_with("application/json")
    } else {
        false
    }
}

/// normalize url path
pub fn normalize_url_path(url_path: &str, url_path_prefix: Option<&str>) -> String {
    let url_path_prefix = match url_path_prefix {
        Some(prefix) if !prefix.is_empty() => prefix.strip_suffix("/").unwrap_or_else(|| prefix),
        _ => "",
    };

    let url_path = url_path.strip_prefix("/").unwrap_or_else(|| url_path);

    let merged = format!("{}/{}", url_path_prefix, url_path);

    let mut ret: &str = merged.as_str();
    ret = ret.strip_suffix("/").unwrap_or_else(|| ret);
    ret = ret.strip_prefix("/").unwrap_or_else(|| ret);
    format!("/{}", ret)
}

/// sleep
pub async fn delay_response(milliseconds: u16) {
    time::sleep(Duration::from_millis(milliseconds.into())).await
}

/// check if file is json
pub fn file_is_json(p: &Path) -> bool {
    JSON_COMPATIBLE_EXTENSIONS.contains(
        &p.extension()
            .unwrap_or_default()
            .to_ascii_lowercase()
            .to_str()
            .unwrap_or_default(),
    )
}

/// check if two json files are equivalent to each
pub fn is_equivalent_json_file(request_path: &Path, entry_path: &Path) -> bool {
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
        && JSON_COMPATIBLE_EXTENSIONS
            .contains(&request_ext.to_str().expect("failed to get requestfile ext"))
        && JSON_COMPATIBLE_EXTENSIONS
            .contains(&entry_ext.to_str().expect("failed to get entry file ext"))
}
