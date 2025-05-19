use std::path::Path;

use serde_json::Value;

pub const JSON_COMPATIBLE_EXTENSIONS: [&str; 3] = ["json", "json5", "csv"];

/// resolve unknown path with json compatible extensions supplied
pub fn resolve_with_json_compatible_extensions(unknown_path: &str) -> Option<String> {
    let p = Path::new(unknown_path);
    if p.is_file() {
        return Some(unknown_path.to_owned());
    }

    let ext = JSON_COMPATIBLE_EXTENSIONS
        .iter()
        .find(|ext| Path::new(format!("{}.{}", unknown_path, ext).as_str()).is_file());
    match ext {
        Some(ext) => Some(format!("{}.{}", unknown_path, ext)),
        None => None,
    }
}

/// check if two json files are equivalent to each
pub fn is_equivalent_json_file(request_path: &Path, entry_path: &Path) -> bool {
    let request_file_stem = request_path
        .file_stem()
        .expect("failed to get request file stem");
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
        && JSON_COMPATIBLE_EXTENSIONS.contains(
            &request_ext
                .to_str()
                .expect("failed to get request file ext"),
        )
        && JSON_COMPATIBLE_EXTENSIONS
            .contains(&entry_ext.to_str().expect("failed to get entry file ext"))
}

/// get json value by jsonpath key
pub fn json_value_by_jsonpath<'a>(value: &'a Value, jsonpath: &str) -> Option<&'a Value> {
    let ret = jsonpath
        .split('.')
        .fold(Some(value), |current, key| match current {
            Some(Value::Object(map)) => map.get(key),
            Some(Value::Array(arr)) => key.parse::<usize>().ok().and_then(|i| arr.get(i)),
            _ => None,
        });
    ret
}
