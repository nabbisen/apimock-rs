use core::fmt;

use serde::Deserialize;

use crate::core::util::glob::glob_match;

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RuleOp {
    Equal,
    NotEqual,
    StartsWith,
    Contains,
    WildCard,
}

impl RuleOp {
    pub fn is_match(&self, text: &str, checker: &str) -> bool {
        match self {
            Self::Equal => text == checker,
            Self::NotEqual => text != checker,
            Self::StartsWith => text.starts_with(checker),
            Self::Contains => text.contains(checker),
            Self::WildCard => glob_match(checker, text),
        }
    }
}
