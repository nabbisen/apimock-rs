use serde::Deserialize;
use strategy::Strategy;
use util::canonicalized_fallback_respond_dir;

use std::path::Path;

pub mod strategy;
mod util;

use super::constant::{
    PRINT_DELIMITER, SERVICE_DEFAULT_FALLBACK_RESPOND_DIR, SERVICE_DEFAULT_RULE_SET_FILE_PATH,
};
use crate::core::server::routing::rule_set::RuleSet;

/// verbose logs
#[derive(Clone, Deserialize)]
pub struct ServiceConfig {
    // routing
    #[serde(rename = "rule_sets")]
    pub rule_sets_file_paths: Vec<String>,
    #[serde(skip)]
    pub rule_sets: Vec<RuleSet>,
    pub strategy: Option<Strategy>,

    pub fallback_respond_dir: String,
}

impl ServiceConfig {
    pub fn validate(&self) -> bool {
        let rule_sets_validate =
            self.rule_sets
                .iter()
                .enumerate()
                .all(|(rule_set_idx, rule_set)| {
                    let prefix_validate =
                        rule_set.prefix.is_none() || rule_set.prefix.as_ref().unwrap().validate();

                    let default_validate =
                        rule_set.default.is_none() || rule_set.default.as_ref().unwrap().validate();

                    let guard_validate =
                        rule_set.guard.is_none() || rule_set.guard.as_ref().unwrap().validate();

                    let dir_prefix = rule_set.dir_prefix();
                    let rules_validate =
                        rule_set.rules.iter().enumerate().all(|(rule_idx, rule)| {
                            rule.when.validate()
                                && rule.respond.validate(
                                    dir_prefix.as_str(),
                                    rule_idx,
                                    rule_set_idx,
                                )
                        });

                    prefix_validate && default_validate && guard_validate && rules_validate
                });

        let fallback_respond_dir_validate = Path::new(self.fallback_respond_dir.as_str()).exists();

        rule_sets_validate && fallback_respond_dir_validate
    }
}

impl Default for ServiceConfig {
    fn default() -> Self {
        let rule_sets_file_paths = if Path::new(SERVICE_DEFAULT_RULE_SET_FILE_PATH).exists() {
            vec![SERVICE_DEFAULT_RULE_SET_FILE_PATH.to_owned()]
        } else {
            vec![]
        };

        ServiceConfig {
            rule_sets_file_paths,
            rule_sets: vec![],
            strategy: Some(Strategy::default()),
            fallback_respond_dir: SERVICE_DEFAULT_FALLBACK_RESPOND_DIR.to_owned(),
        }
    }
}

impl std::fmt::Display for ServiceConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let has_rule_sets = 0 < self.rule_sets.iter().len();

        if has_rule_sets {
            let _ = writeln!(
                f,
                "@ rule sets strategy = {}",
                self.strategy.clone().unwrap_or_default()
            );
        }

        for (idx, rule_set) in self.rule_sets.iter().enumerate() {
            let _ = writeln!(f, "");
            let _ = writeln!(f, "@ rule_set #{}\n", idx + 1);
            let _ = write!(f, "{}", rule_set);
        }

        if has_rule_sets {
            let _ = writeln!(f, "{}", PRINT_DELIMITER);
        }

        let _ = writeln!(
            f,
            "[fallback_respond_dir] {}",
            canonicalized_fallback_respond_dir(self.fallback_respond_dir.as_str())
        );

        Ok(())
    }
}
