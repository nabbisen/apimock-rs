use std::path::Path;

/// full file path by joining path prefix to file path
pub fn full_file_path(path: &str, dir_prefix: &str) -> Option<String> {
    let p = if !dir_prefix.is_empty() {
        Path::new(dir_prefix).join(path)
    } else {
        Path::new(path).to_path_buf()
    };

    if !p.exists() {
        return None;
    }

    match p.to_str() {
        Some(x) => Some(x.to_owned()),
        None => {
            log::error!(
                "faild to get str from canonicalized url path: {} (prefix = {})",
                path,
                dir_prefix
            );
            None
        }
    }
}
