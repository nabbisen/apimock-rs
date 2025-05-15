use serde::Deserialize;
use util::{body_is_match, headers_is_match, url_path_is_match};

mod request;
mod util;

use super::types::RuleOp;
use crate::core::server::parsed_request::ParsedRequest;
use request::Request;

#[derive(Clone, Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BodyKind {
    Json,
}

#[derive(Clone, Deserialize, Debug)]
pub struct When {
    pub url_path: Option<String>,
    pub url_path_op: Option<RuleOp>,
    pub request: Option<Request>,
}

impl When {
    pub fn validate(&self) -> bool {
        if self.url_path.is_some() {
            return true;
        }

        match self.request.as_ref() {
            Some(request) => request.headers.is_some() || request.body.is_some(),
            None => false,
        }
    }

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
}
