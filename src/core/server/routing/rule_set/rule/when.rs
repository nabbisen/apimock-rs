use console::style;
use rule_op::RuleOp;
use serde::Deserialize;
use util::{body_is_match, headers_is_match, url_path_is_match};

mod condition_statement;
mod request;
mod rule_op;
mod util;

use crate::core::server::parsed_request::ParsedRequest;
use request::Request;

#[derive(Clone, Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BodyKind {
    Json,
}

impl std::fmt::Display for BodyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyKind::Json => write!(f, "JSON"),
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct When {
    pub url_path: Option<String>,
    pub url_path_op: Option<RuleOp>,
    pub request: Option<Request>,
}

impl std::fmt::Display for When {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.url_path_op.is_some() {
            let _ = write!(
                f,
                "[url_path]{}{}",
                self.url_path_op.as_ref().unwrap(),
                style(self.url_path.as_ref().unwrap()).yellow(),
            );
        }

        if self.request.is_some() {
            let _ = write!(f, "{}", self.request.as_ref().unwrap());
        }

        Ok(())
    }
}

impl When {
    /// match with condition
    pub fn is_match(&self, request: &ParsedRequest, rule_idx: usize, rule_set_idx: usize) -> bool {
        if let Some(matcher_url_path) = self.url_path.as_ref() {
            if !url_path_is_match(
                request.uri_path.as_str(),
                matcher_url_path,
                self.url_path_op.as_ref(),
            ) {
                return false;
            }
        }

        if let Some(matcher) = self.request.as_ref() {
            if let Some(matcher_headers) = matcher.headers.as_ref() {
                if !headers_is_match(
                    &request.component_parts.headers,
                    matcher_headers,
                    rule_idx,
                    rule_set_idx,
                ) {
                    return false;
                }
            }

            if let Some(matcher_body) = matcher.body.as_ref() {
                if !body_is_match(request, matcher_body, rule_idx, rule_set_idx) {
                    return false;
                }
            }
        }

        true
    }

    /// validate
    pub fn validate(&self) -> bool {
        let request_validate = match self.request.as_ref() {
            Some(request) => request.headers.is_some() || request.body.is_some(),
            None => true,
        };

        request_validate
    }
}
