use serde::Deserialize;
use util::{fallback_response_dir_print, rule_sets_print};

use std::path::Path;

mod util;

use super::constant::{SERVICE_DEFAULT_FALLBACK_RESPONSE_DIR, SERVICE_DEFAULT_RULE_SET_FILE_PATH};
use crate::core::server::routing::rule_set::RuleSet;

#[derive(Clone, Deserialize)]
pub enum Strategy {
    FirstMatch,
}

impl Default for Strategy {
    fn default() -> Self {
        Self::FirstMatch
    }
}

/// verbose logs
#[derive(Clone, Deserialize)]
pub struct ServiceConfig {
    // routing
    #[serde(rename = "rule_sets")]
    pub rule_sets_file_paths: Vec<String>,
    #[serde(skip)]
    pub rule_sets: Vec<RuleSet>,
    pub strategy: Option<Strategy>,

    pub fallback_response_dir: String,
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
            fallback_response_dir: SERVICE_DEFAULT_FALLBACK_RESPONSE_DIR.to_owned(),
        }
    }
}

impl ServiceConfig {
    pub fn print(&self) {
        rule_sets_print(self.rule_sets.as_ref());
        fallback_response_dir_print(self.fallback_response_dir.as_str());
    }
}
