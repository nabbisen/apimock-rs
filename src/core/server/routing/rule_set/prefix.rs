use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Prefix {
    pub dir_prefix: Option<String>,
    pub url_path_prefix: Option<String>,
}
