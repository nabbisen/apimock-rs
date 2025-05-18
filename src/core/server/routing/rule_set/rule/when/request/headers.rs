use hyper::{header::HeaderValue, HeaderMap};
use serde::Deserialize;

use std::collections::HashMap;

use crate::core::server::routing::rule_set::rule::{
    when::condition_statement::ConditionStatement, ConditionKey,
};

#[derive(Clone, Debug, Deserialize)]
#[serde(transparent)]
pub struct Headers(pub HashMap<ConditionKey, ConditionStatement>);

impl Headers {
    /// check if `headers` in `when` matches
    pub fn is_match(
        &self,
        sent_request_headers: &HeaderMap<HeaderValue>,
        rule_idx: usize,
        rule_set_idx: usize,
    ) -> bool {
        if self
            .0
            .iter()
            .any(|(matcher_header_key, matcher_header_value)| {
                let sent_request_header_value = sent_request_headers.get(matcher_header_key);
                if sent_request_header_value.is_none() {
                    return true;
                }
                let sent_request_header_value = match sent_request_header_value.unwrap().to_str() {
                    Ok(x) => x,
                    Err(err) => {
                        log::error!(
                            "failed to get value from {} (rule #{} in rule set #{}) ({})",
                            matcher_header_key,
                            rule_idx + 1,
                            rule_set_idx + 1,
                            err
                        );
                        return true;
                    }
                };
                let op = matcher_header_value.op.clone().unwrap_or_default();
                !op.is_match(sent_request_header_value, &matcher_header_value.value)
            })
        {
            return false;
        }
        true
    }

    /// validate
    pub fn validate(&self) -> bool {
        0 < self.0.len()
    }
}

impl std::fmt::Display for Headers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (header_key, header_statement) in self.0.iter() {
            let _ = write!(f, "{}{}", header_key, header_statement);
        }

        Ok(())
    }
}
