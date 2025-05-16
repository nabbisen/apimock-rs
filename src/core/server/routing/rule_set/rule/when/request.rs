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

impl Request {
    pub fn print(&self) {
        if self.headers.is_some() {
            // todo: print()
            // for (key, value) in self.headers.as_ref().unwrap().iter() {
            //     log::info!("[[key]] {}", key);
            //     log::info!("[[value]] {}", value);
            // }
        }

        if self.body.is_some() {
            // todo: print()
            // for (key, value) in self.body.as_ref().unwrap().iter() {
            //     log::info!("[[key]] {}", key);
            //     log::info!("[[value]] {}", value);
            // }
        }
    }
}
