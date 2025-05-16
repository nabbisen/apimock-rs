use std::path::Path;

use serde::Deserialize;

#[derive(Clone, Default, Deserialize, Debug)]
pub struct Prefix {
    #[serde(rename = "url_path")]
    pub url_path_prefix: Option<String>,
    #[serde(rename = "respond_dir")]
    pub respond_dir_prefix: Option<String>,
}

impl std::fmt::Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let has_written = self.url_path_prefix.is_some() || self.respond_dir_prefix.is_some();

        if self.url_path_prefix.is_some() {
            let _ = writeln!(
                f,
                "[url_path_prefix] {}",
                self.url_path_prefix.as_ref().unwrap()
            );
        }

        if self.respond_dir_prefix.is_some() {
            let _ = writeln!(
                f,
                "[respond_dir_prefix] {}",
                self.respond_dir_prefix.as_ref().unwrap()
            );
        }

        if has_written {
            let _ = writeln!(f, "");
        }

        Ok(())
    }
}

impl Prefix {
    /// validate
    pub fn validate(&self) -> bool {
        let respond_dir_prefix_validate =
            if let Some(respond_dir_prefix) = self.respond_dir_prefix.as_ref() {
                let exists = Path::new(respond_dir_prefix.as_str()).exists();
                if !exists {
                    log::error!(
                        "directory `{}` does not exist",
                        self.respond_dir_prefix.clone().unwrap().as_str()
                    );
                }
                exists
            } else {
                true
            };
        respond_dir_prefix_validate
    }
}
