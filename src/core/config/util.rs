use std::{fs, path::Path};

/// api url full path
pub fn fullpath(path: &str, path_prefix: &Option<String>, is_raw_paths: bool) -> String {
    let possibly_w_trailing_slash = if is_raw_paths {
        format!("/{}/", path.to_string())
    } else {
        if let Some(path_prefix) = path_prefix {
            format!("/{}/{}/", path_prefix, path.to_string())
        } else {
            format!("/{}/", path.to_string())
        }
    }
    .replace("//", "/");

    (&possibly_w_trailing_slash[..possibly_w_trailing_slash.len() - 1]).to_owned()
}

/// `data_src` path on static json responses
pub fn data_src_path(file: &str, data_dir: &Option<String>) -> String {
    let data_dir = if let Some(x) = data_dir.clone() {
        x.to_owned()
    } else {
        String::new()
    };
    let path = Path::new(data_dir.as_str())
        .join(file)
        .display()
        .to_string();
    let _ = fs::metadata(&path).expect(format!("`{}` is missing", path).as_str());
    path
}
