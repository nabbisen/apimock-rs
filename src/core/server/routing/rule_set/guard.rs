use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Guard {}

impl Guard {
    pub fn print(&self) {}
}
