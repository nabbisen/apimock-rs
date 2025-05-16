use serde::Deserialize;

use std::collections::HashMap;

pub mod body_condition;

use crate::core::server::routing::rule_set::rule::ConditionKey;

use super::{condition_statement::ConditionStatement, BodyKind};
use body_condition::BodyCondition;

#[derive(Clone, Deserialize, Debug)]
pub struct Request {
    pub headers: Option<HashMap<ConditionKey, ConditionStatement>>,
    pub body: Option<HashMap<BodyKind, BodyCondition>>,
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.headers.is_some() {
            for (header_key, header_statement) in self.headers.as_ref().unwrap().iter() {
                let _ = write!(f, "{}{}", header_key, header_statement);
            }
        }

        if self.body.is_some() {
            for (body_kind, body_condition) in self.body.as_ref().unwrap() {
                let _ = write!(f, "[{}] {}", body_kind, body_condition);
            }
        }

        Ok(())
    }
}
