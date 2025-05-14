use hyper::http::Error;
use serde::Deserialize;

use std::fs;

mod guard;
mod prefix;
pub mod rule;
mod rule_set_default;

use guard::Guard;
use prefix::Prefix;
use rule::Rule;
use rule_set_default::RuleSetDefault;

use crate::core::server::types::BoxBody;

#[derive(Clone, Deserialize, Debug)]
pub struct RuleSet {
    pub prefix: Option<Prefix>,
    pub default: Option<RuleSetDefault>,
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
}

/// handle on `rule_sets`
pub fn rule_sets_content(
    uri_path: &str,
    rule_sets: &Vec<RuleSet>,
) -> Result<Option<hyper::Response<BoxBody>>, hyper::http::Error> {
    Ok(None)
}
