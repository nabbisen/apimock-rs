use std::collections::HashMap;

use body_condition::BodyCondition;
use serde::Deserialize;

use super::types::{ConditionKey, ConditionStatement};

mod body_condition;

#[derive(Clone, Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BodyKind {
    Json,
}

#[derive(Clone, Deserialize, Debug)]
pub struct When {
    pub url_path: Option<String>,
    pub request: Option<Request>,
}

impl When {
    pub fn validate(&self) -> bool {
        if self.url_path.is_some() {
            return true;
        }

        match self.request.as_ref() {
            Some(request) => request.headers.is_some() || request.body.is_some(),
            None => false,
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct Request {
    pub headers: Option<HashMap<ConditionKey, ConditionStatement>>,
    pub body: Option<HashMap<BodyKind, BodyCondition>>,
}
