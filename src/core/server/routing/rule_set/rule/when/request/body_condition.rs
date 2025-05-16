use serde::Deserialize;

use std::collections::HashMap;

use crate::core::server::routing::rule_set::rule::{
    when::condition_statement::ConditionStatement, ConditionKey,
};

#[derive(Clone, Deserialize, Debug)]
pub struct BodyCondition {
    pub json_condition: Option<HashMap<ConditionKey, ConditionStatement>>,
}

impl BodyCondition {
    pub fn print(&self) {
        if self.json_condition.is_some() {
            // todo: print()
            // for (key, value) in self.json_condition.as_ref().unwrap().iter() {
            //     log::info!("[[body.json.key]]", key);
            //     log::info!("[[body.json.statement]]", value);
            // }
        }
    }
}
