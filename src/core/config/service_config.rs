use console::style;
use serde::Deserialize;

use std::{fs, path::Path};

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

impl std::fmt::Display for ServiceConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rule_set in self.rule_sets.iter() {
            let _ = writeln!(f, "{}", rule_set);
        }
        let _ = write!(
            f,
            "[fallback_response_dir] {}",
            canonicalized_fallback_response_dir(self.fallback_response_dir.as_str())
        );
        Ok(())
    }
}

/// canonicalized fallback_response_dir
fn canonicalized_fallback_response_dir(fallback_response_dir: &str) -> String {
    let p = Path::new(fallback_response_dir);
    if p.is_relative() {
        let absolute_path = fs::canonicalize(fallback_response_dir)
            .expect(format!("{} does not exist", fallback_response_dir).as_str());
        format!(
            "{} ({})",
            style(fallback_response_dir).green(),
            absolute_path
                .to_str()
                .expect(format!("logger failed to print out: {}", fallback_response_dir).as_str())
        )
    } else {
        format!("{}", style(fallback_response_dir).green().to_string())
    }
}
