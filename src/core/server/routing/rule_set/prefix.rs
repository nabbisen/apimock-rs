use serde::Deserialize;

#[derive(Clone, Default, Deserialize, Debug)]
pub struct Prefix {
    pub dir_prefix: Option<String>,
    pub url_path_prefix: Option<String>,
}

impl std::fmt::Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.dir_prefix.is_some() {
            let _ = write!(f, "[[ dir_prefix ]] {}", self.dir_prefix.as_ref().unwrap());
        }

        if self.url_path_prefix.is_some() {
            let _ = write!(
                f,
                "[[ url_path_prefix ]] {}",
                self.url_path_prefix.as_ref().unwrap()
            );
        }

        Ok(())
    }
}
