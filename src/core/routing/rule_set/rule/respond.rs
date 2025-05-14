use std::collections::HashMap;

use serde::Deserialize;

mod util;

#[derive(Clone, Deserialize, Debug)]
pub enum ResponseType {
    File,
    Text,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Respond {
    pub response_type: Option<ResponseType>,
    pub headers: Option<HashMap<String, Option<String>>>,
    pub code: Option<u16>,
    pub content: String,
    pub wait_milliseconds: Option<u16>,
}
