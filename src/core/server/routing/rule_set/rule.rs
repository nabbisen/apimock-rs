use console::style;
use hyper::StatusCode;
use serde::Deserialize;

pub mod respond;
mod util;
pub mod when;

use super::RuleSet;
use respond::Respond;
use util::url_path_with_prefix;
use when::{
    request::url_path::{UrlPath, UrlPathConfig},
    When,
};

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
        let url_path = match ret.when.request.url_path_config.as_ref() {
            Some(url_path_config) => match url_path_config {
                UrlPathConfig::Simple(s) => Some(UrlPath {
                    value: s.clone(),
                    value_with_prefix: url_path_with_prefix(s.as_str(), rule_set.prefix.as_ref()),
                    op: None,
                }),

                UrlPathConfig::Detailed(url_path) => Some(UrlPath {
                    value: url_path.value.clone(),
                    value_with_prefix: url_path_with_prefix(
                        url_path.value.as_str(),
                        rule_set.prefix.as_ref(),
                    ),
                    op: url_path.op.clone(),
                }),
            },
            None => None,
        };
        ret.when.request.url_path = url_path;

        // - status_code
        if let Some(code) = ret.respond.code {
            let status_code = Some(
                StatusCode::from_u16(code).expect(
                    format!(
                        "failed to get status code from code {} (rule #{} in rule set #{})",
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
        self.when.validate(rule_idx, rule_set_idx)
            && self.respond.validate(dir_prefix, rule_idx, rule_set_idx)
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "- ");
        let _ = write!(f, "{} {}", style("[when]").yellow(), self.when);
        let _ = write!(f, "{} {}", style("[respond]").yellow(), self.respond);
        Ok(())
    }
}
