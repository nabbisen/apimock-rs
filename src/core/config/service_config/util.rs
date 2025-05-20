use console::style;

use std::{fs, path::Path};

/// canonicalized fallback_respond_dir
pub fn canonicalized_fallback_respond_dir_to_print(fallback_respond_dir: &str) -> String {
    let p = Path::new(fallback_respond_dir);
    if p.is_relative() {
        let absolute_path = fs::canonicalize(fallback_respond_dir)
            .expect(format!("{} does not exist", fallback_respond_dir).as_str());
        format!(
            "{} ({})",
            style(fallback_respond_dir).green(),
            absolute_path
                .to_str()
                .expect(format!("logger failed to print out: {}", fallback_respond_dir).as_str())
        )
    } else {
        format!("{}", style(fallback_respond_dir).green().to_string())
    }
}
