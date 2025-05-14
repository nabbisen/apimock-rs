mod general;
pub mod rule;

use std::fs;

use general::General;
use rule::Rule;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct RuleSet {
    pub general: Option<General>,
    pub rule: Vec<Rule>,
}

impl RuleSet {
    pub fn new(ruleset_filepath: &str) -> Self {
        let toml_string = fs::read_to_string(ruleset_filepath).unwrap();
        let deserialized = toml::from_str(&toml_string);
        match deserialized {
            Ok(x) => x,
            Err(err) => panic!("{}: Invalid toml content\n({})", ruleset_filepath, err),
        }
    }
}
