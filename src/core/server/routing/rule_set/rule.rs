use respond::Respond;
use serde::Deserialize;
use when::When;

pub mod respond;
pub mod when;

type ConditionKey = String;

#[derive(Clone, Deserialize, Debug)]
pub struct Rule {
    pub when: When,
    pub respond: Respond,
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "{}", self.when);
        let _ = write!(f, " => {}", self.respond);
        Ok(())
    }
}
