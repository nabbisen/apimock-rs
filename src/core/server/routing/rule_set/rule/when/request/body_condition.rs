use serde::Deserialize;

use std::collections::HashMap;

use crate::core::server::routing::rule_set::rule::{
    when::condition_statement::ConditionStatement, ConditionKey,
};

#[derive(Clone, Deserialize, Debug)]
pub struct BodyCondition {
    pub condition: Option<HashMap<ConditionKey, ConditionStatement>>,
}

impl std::fmt::Display for BodyCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.condition.is_none() {
            return Ok(());
        }
        for (key, statement) in self.condition.as_ref().unwrap().iter() {
            let _ = write!(f, "{} {}", key, statement);
        }
        Ok(())
    }
}
