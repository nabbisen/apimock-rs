use core::fmt;

use serde::Deserialize;

use super::rule_op::RuleOp;

#[derive(Clone, Deserialize, Debug)]
pub struct ConditionStatement {
    pub op: RuleOp,
    pub value: String,
}
