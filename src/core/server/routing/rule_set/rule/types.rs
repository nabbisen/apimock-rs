use globset::Glob;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RuleOp {
    Equal,
    NotEqual,
    StartsWith,
    Contains,
    WildCard,
}

pub type ConditionKey = String;

#[derive(Clone, Deserialize, Debug)]
pub struct ConditionStatement {
    pub op: RuleOp,
    pub value: String,
}

impl RuleOp {
    pub fn is_match(&self, text: &str, checker: &str) -> bool {
        match self {
            Self::Equal => text == checker,
            Self::NotEqual => text != checker,
            Self::StartsWith => text.starts_with(checker),
            Self::Contains => text.contains(checker),
            Self::WildCard => match Glob::new(checker) {
                Ok(glob) => glob.compile_matcher().is_match(text),
                Err(err) => {
                    log::error!(
                        "failed to match with wild card pattern: {}\n({})",
                        checker,
                        err
                    );
                    return false;
                }
            },
        }
    }
}
