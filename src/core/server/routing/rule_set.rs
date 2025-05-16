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
    config::service_config::Strategy,
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
    pub fn new(rule_set_file_path: &str) -> Self {
        let toml_string = fs::read_to_string(rule_set_file_path).unwrap();
        let deserialized = toml::from_str(&toml_string);
        match deserialized {
            Ok(x) => x,
            Err(err) => panic!("{}: Invalid toml content\n({})", rule_set_file_path, err),
        }
    }

    pub fn validate(&self) -> bool {
        true
    }

    pub fn print(&self) {
        if self.prefix.is_some() {
            self.prefix.as_ref().unwrap().print();
        }
        if self.guard.is_some() {
            self.guard.as_ref().unwrap().print();
        }
        if self.default.is_some() {
            self.default.as_ref().unwrap().print();
        }
        for rule in self.rules.iter().as_ref() {
            rule.print();
        }
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
            if rule.when.is_match(request, rule_idx, rule_set_idx) {
                let response = rule
                    .respond
                    .response(rule_set.prefix.clone().unwrap_or_default().dir_prefix)
                    .await;

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
