use std::collections::HashMap;

use serde::Deserialize;

use crate::core::server::routing::rule_set::rule::types::{ConditionKey, ConditionStatement};

#[derive(Clone, Deserialize, Debug)]
pub struct BodyCondition {
    pub json_condition: Option<HashMap<ConditionKey, ConditionStatement>>,
}
