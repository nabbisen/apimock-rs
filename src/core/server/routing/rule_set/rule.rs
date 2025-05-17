use hyper::StatusCode;
use respond::Respond;
use serde::Deserialize;
use util::url_path_with_prefix;
use when::When;

use super::RuleSet;

pub mod respond;
mod util;
pub mod when;

type ConditionKey = String;

#[derive(Clone, Deserialize, Debug)]
pub struct Rule {
    pub when: When,
    pub respond: Respond,
}

impl Rule {
    pub fn compute_derived_fields(
        &self,
        rule_set: &RuleSet,
        rule_idx: usize,
        rule_set_idx: usize,
    ) -> Self {
        let mut ret = self.to_owned();

        // - url_path_with_prefix
        if let Some(url_path) = ret.when.url_path.as_ref() {
            ret.when.url_path_with_prefix = Some(url_path_with_prefix(
                url_path.as_str(),
                rule_set.prefix.as_ref(),
            ));
        }

        // - status_code
        if let Some(code) = ret.respond.code {
            let status_code = Some(
                StatusCode::from_u16(code).expect(
                    format!(
                        "failed to get status code from code {}\n(rule #{} in rule set #{})",
                        code,
                        rule_idx + 1,
                        rule_set_idx + 1
                    )
                    .as_str(),
                ),
            );
            ret.respond.status_code = status_code;
        }

        ret
    }

    pub fn validate(&self, dir_prefix: &str, rule_idx: usize, rule_set_idx: usize) -> bool {
        self.when.validate() && self.respond.validate(dir_prefix, rule_idx, rule_set_idx)
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "{}", self.when);
        let _ = write!(f, "{}", self.respond);
        Ok(())
    }
}
