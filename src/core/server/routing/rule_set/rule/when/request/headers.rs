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
        self.0
            .iter()
            .all(|(matcher_header_key, matcher_header_value)| {
                let sent_request_header_value = match sent_request_headers.get(matcher_header_key) {
                    Some(x) => x,
                    None => return false,
                };

                let sent_request_header_value = match sent_request_header_value.to_str() {
                    Ok(x) => x,
                    Err(err) => {
                        log::error!(
                            "failed to get request header value by key `{}` (rule #{} in rule set #{}) ({})",
                            matcher_header_key,
                            rule_idx + 1,
                            rule_set_idx + 1,
                            err
                        );
                        return true;
                    }
                };

                let ret = matcher_header_value
                    .op
                    .clone()
                    .unwrap_or_default()
                    .is_match(sent_request_header_value, &matcher_header_value.value);
                ret
            })
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
