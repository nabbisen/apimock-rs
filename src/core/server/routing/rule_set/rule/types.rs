use serde::Deserialize;

pub type ConditionKey = String;

#[derive(Clone, Deserialize, Debug)]
pub struct ConditionStatement {
    op: RuleOp,
    value: String,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RuleOp {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Contains,
    WildCars,
}
