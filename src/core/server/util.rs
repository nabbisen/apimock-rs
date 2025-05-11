use std::path::Path;

use super::constant::JSON_EXTENSIONS;

/// check if file is json
pub fn file_is_json(p: &Path) -> bool {
    JSON_EXTENSIONS.contains(
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
        && JSON_EXTENSIONS.contains(&request_ext.to_str().expect("failed to get requestfile ext"))
        && JSON_EXTENSIONS.contains(&entry_ext.to_str().expect("failed to get entry file ext"))
}
