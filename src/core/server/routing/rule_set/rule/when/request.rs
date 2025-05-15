use serde::Deserialize;

use std::collections::HashMap;

pub mod body_condition;

use super::BodyKind;
use crate::core::server::routing::rule_set::rule::types::{ConditionKey, ConditionStatement};
use body_condition::BodyCondition;

#[derive(Clone, Deserialize, Debug)]
pub struct Request {
    pub headers: Option<HashMap<ConditionKey, ConditionStatement>>,
    pub body: Option<HashMap<BodyKind, BodyCondition>>,
}
