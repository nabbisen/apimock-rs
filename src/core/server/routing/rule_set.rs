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

#[derive(Clone, Deserialize, Debug)]
pub struct RuleSet {
    pub prefix: Option<Prefix>,
    pub default: Option<DefaultRespond>,
    pub guard: Option<Guard>,
    pub rules: Vec<Rule>,
    #[serde(skip)]
    pub file_path: String,
}

impl RuleSet {
    /// create instance
    pub fn new(rule_set_file_path: &str, rule_set_idx: usize) -> Self {
        let toml_string = fs::read_to_string(rule_set_file_path)
            .expect(format!("failed to read rule set toml `{}`", rule_set_file_path).as_str());
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

        ret.file_path = rule_set_file_path.to_owned();

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
        if let Some(x) = self.prefix.as_ref() {
            let _ = write!(f, "{}", x);
        }
        if let Some(x) = self.guard.as_ref() {
            let _ = write!(f, "{}", x);
        }
        if let Some(x) = self.default.as_ref() {
            let _ = write!(f, "{}", x);
        }
        for rule in self.rules.iter() {
            let _ = write!(f, "{}", rule);
        }
        Ok(())
    }
}
