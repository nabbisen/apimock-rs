use serde::Deserialize;

#[derive(Clone, Default, Deserialize, Debug)]
pub struct Prefix {
    pub dir_prefix: Option<String>,
    pub url_path_prefix: Option<String>,
}

impl Prefix {
    pub fn print(&self) {
        if self.dir_prefix.is_some() {
            log::info!("[[ dir_prefix ]] {}", self.dir_prefix.as_ref().unwrap());
        }

        if self.url_path_prefix.is_some() {
            log::info!(
                "[[ url_path_prefix ]] {}",
                self.url_path_prefix.as_ref().unwrap()
            );
        }
    }
}
