use std::collections::HashMap;

use serde::Deserialize;

use super::super::types::{ConditionKey, ConditionStatement};

#[derive(Clone, Deserialize, Debug)]
pub struct BodyCondition {
    pub json_condition: Option<HashMap<ConditionKey, ConditionStatement>>,
}
