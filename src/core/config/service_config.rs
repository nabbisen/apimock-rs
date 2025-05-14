use serde::Deserialize;

use crate::core::routing::rule_set::RuleSet;

use super::constant::SERVICE_DEFAULT_FALLBACK_RESPONSE_DIR;

/// verbose logs
#[derive(Clone, Deserialize)]
pub struct ServiceConfig {
    // routing
    #[serde(rename = "rule_sets")]
    pub rule_sets_file_paths: Vec<String>,
    #[serde(skip)]
    pub rule_sets: Vec<RuleSet>,

    pub fallback_response_dir: String,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        ServiceConfig {
            rule_sets_file_paths: vec![],
            rule_sets: vec![],
            fallback_response_dir: SERVICE_DEFAULT_FALLBACK_RESPONSE_DIR.to_owned(),
        }
    }
}
