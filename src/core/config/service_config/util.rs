use console::style;

use std::{fs, path::Path};

use crate::core::server::routing::rule_set::RuleSet;

pub fn rule_sets_print(rule_sets: &Vec<RuleSet>) {
    for rule_set in rule_sets {
        rule_set.print();
    }
}

pub fn fallback_response_dir_print(fallback_response_dir: &str) {
    let p = Path::new(fallback_response_dir);
    let p = if p.is_relative() {
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
        style(fallback_response_dir).green().to_string()
    };
    log::info!("[service.fallback_response_dir] {}", p);
}
