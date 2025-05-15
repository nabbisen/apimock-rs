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

use crate::core::server::{
    parsed_request::ParsedRequest, types::BoxBody,
};

#[derive(Clone, Deserialize, Debug)]
pub struct RuleSet {
    pub prefix: Option<Prefix>,
    pub default: Option<DefaultRespond>,
    pub guard: Option<Guard>,
    pub rules: Vec<Rule>,
}

impl RuleSet {
    pub fn new(ruleset_file_path: &str) -> Self {
        let toml_string = fs::read_to_string(ruleset_file_path).unwrap();
        let deserialized = toml::from_str(&toml_string);
        match deserialized {
            Ok(x) => x,
            Err(err) => panic!("{}: Invalid toml content\n({})", ruleset_file_path, err),
        }
    }

    pub fn validate() -> bool {
        true
    }
}

/// handle on `rule_sets`
pub async fn rule_sets_content(
    request: &ParsedRequest,
    rule_sets: &Vec<RuleSet>,
) -> Option<Result<hyper::Response<BoxBody>, hyper::http::Error>> {
    for (rule_set_idx, rule_set) in rule_sets.iter().enumerate() {
        for (rule_idx, rule) in rule_set.rules.iter().enumerate() {
            if rule.when.is_match(request, rule_idx, rule_set_idx) {
                let response = rule.respond.response().await;
                return Some(response);
            }
        }
    }

    None
}
