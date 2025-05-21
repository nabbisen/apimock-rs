use default_respond::DefaultRespond;
use serde::Deserialize;

use std::{fs, path::Path};

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
    pub fn new(
        rule_set_file_path: &str,
        current_dir_to_config_dir_relative_path: &str,
        rule_set_idx: usize,
    ) -> Self {
        let toml_string = fs::read_to_string(rule_set_file_path)
            .expect(format!("failed to read rule set toml `{}`", rule_set_file_path).as_str());
        let deserialized = toml::from_str::<Self>(&toml_string);
        let mut ret = match deserialized {
            Ok(x) => x,
            Err(err) => panic!("{}: invalid toml content\n({})", rule_set_file_path, err),
        };

        // - prefix - respond_dir_prefix
        let mut prefix = match ret.prefix {
            Some(x) => x.clone(),
            None => Prefix::default(),
        };

        let respond_dir_prefix = match prefix.respond_dir_prefix.as_ref() {
            Some(respond_dir_prefix) => respond_dir_prefix.as_str(),
            None => ".",
        };

        let respond_dir_prefix =
            Path::new(current_dir_to_config_dir_relative_path).join(respond_dir_prefix);
        let respond_dir_prefix = respond_dir_prefix.to_str().expect(
            format!(
                "failed to get path str: {}",
                respond_dir_prefix.to_string_lossy()
            )
            .as_str(),
        );

        prefix.respond_dir_prefix = Some(respond_dir_prefix.to_owned());
        ret.prefix = Some(prefix);

        // - rules
        ret.rules = ret
            .rules
            .iter()
            .enumerate()
            .map(|(rule_idx, rule)| rule.compute_derived_fields(&ret, rule_idx, rule_set_idx))
            .collect();

        // - file path
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
