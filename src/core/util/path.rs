use std::{
    env, io,
    path::{Path, PathBuf},
};

#[cfg(test)]
mod tests;

/// relative path from current dir (working dir) to file parent dir
pub fn current_dir_to_file_parent_dir_relative_path(file_path: &str) -> io::Result<PathBuf> {
    relative_path(
        env::current_dir()?.as_path(),
        Path::new(file_path)
            .parent()
            .expect(&format!("failed to get parent dir: {}", file_path)),
    )
}

/// relative path between two paths
pub fn relative_path(from: &Path, to: &Path) -> io::Result<PathBuf> {
    let from_abs = std::fs::canonicalize(from)?;
    let to_abs = std::fs::canonicalize(to)?;

    let mut from_iter = from_abs.components();
    let mut to_iter = to_abs.components();

    let mut from_rest = vec![];
    let mut to_rest = vec![];
    // collect common prefix
    let mut common_prefix = vec![];
    loop {
        match (from_iter.next(), to_iter.next()) {
            (Some(f), Some(t)) if f == t => {
                common_prefix.push(f);
            }
            (Some(f), Some(t)) => {
                from_rest.push(f);
                to_rest.push(t);
                from_rest.extend(from_iter);
                to_rest.extend(to_iter);
                break;
            }
            (Some(f), None) => {
                from_rest.push(f);
                from_rest.extend(from_iter);
                break;
            }
            (None, Some(t)) => {
                to_rest.push(t);
                to_rest.extend(to_iter);
                break;
            }
            (None, None) => break,
        }
    }

    let mut result = PathBuf::new();

    for _ in from_rest {
        result.push("..");
    }

    for t in to_rest {
        result.push(t.as_os_str());
    }

    if result.as_os_str().is_empty() {
        Ok(PathBuf::from("."))
    } else {
        Ok(result)
    }
}
