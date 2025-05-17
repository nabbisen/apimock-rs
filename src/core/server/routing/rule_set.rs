use default_respond::DefaultRespond;
use serde::Deserialize;

use std::fs;

mod default_respond;
mod guard;
mod prefix;
pub mod rule;

use guard::Guard;
use prefix::Prefix;
use rule::Rule;

use crate::core::{
    config::service_config::strategy::Strategy,
    server::{parsed_request::ParsedRequest, types::BoxBody},
};

#[derive(Clone, Deserialize, Debug)]
pub struct RuleSet {
    pub prefix: Option<Prefix>,
    pub default: Option<DefaultRespond>,
    pub guard: Option<Guard>,
    pub rules: Vec<Rule>,
}

impl RuleSet {
    /// create instance
    pub fn new(rule_set_file_path: &str, rule_set_idx: usize) -> Self {
        let toml_string = fs::read_to_string(rule_set_file_path).unwrap();
        let deserialized = toml::from_str::<Self>(&toml_string);
        let mut ret = match deserialized {
            Ok(x) => x,
            Err(err) => panic!("{}: Invalid toml content\n({})", rule_set_file_path, err),
        };

        ret.rules = ret
            .rules
            .iter()
            .enumerate()
            .map(|(rule_idx, rule)| rule.compute_derived_fields(&ret, rule_idx, rule_set_idx))
            .collect();

        ret
    }

    /// validate
    pub fn validate(&self) -> bool {
        true
    }

    /// dir_prefix as string possibly as empty
    pub fn dir_prefix(&self) -> String {
        if let Some(dir_prefix) = self.prefix.clone().unwrap_or_default().respond_dir_prefix {
            dir_prefix
        } else {
            String::new()
        }
    }
}

impl std::fmt::Display for RuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.prefix.is_some() {
            let _ = write!(f, "{}", self.prefix.as_ref().unwrap());
        }
        if self.default.is_some() {
            let _ = write!(f, "{}", self.default.as_ref().unwrap());
        }
        if self.guard.is_some() {
            let _ = write!(f, "{}", self.guard.as_ref().unwrap());
        }
        for rule in self.rules.iter() {
            let _ = write!(f, "{}", rule);
        }
        Ok(())
    }
}

/// handle on `rule_sets`
pub async fn rule_sets_content(
    request: &ParsedRequest,
    rule_sets: &Vec<RuleSet>,
    strategy: Option<&Strategy>,
) -> Option<Result<hyper::Response<BoxBody>, hyper::http::Error>> {
    for (rule_set_idx, rule_set) in rule_sets.iter().enumerate() {
        for (rule_idx, rule) in rule_set.rules.iter().enumerate() {
            let is_match = rule.when.is_match(request, rule_idx, rule_set_idx);
            if is_match {
                let dir_prefix = rule_set.dir_prefix();

                let response = rule.respond.response(dir_prefix.as_str()).await;

                // todo : last match in the future ?
                match strategy {
                    Some(Strategy::FirstMatch) | None => {
                        return Some(response);
                    }
                }
            }
        }
    }

    None
}
