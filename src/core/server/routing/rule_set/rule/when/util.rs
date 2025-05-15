use std::collections::HashMap;

use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    HeaderMap,
};

use crate::core::server::{
    parsed_request::ParsedRequest,
    routing::rule_set::rule::types::{ConditionStatement, RuleOp},
    util::content_type_is_application_json,
};

use super::{request::body_condition::BodyCondition, BodyKind};

/// check if `url_path` in `when`` matches
pub fn url_path_is_match(
    request_uri_path: &str,
    matcher_url_path: &str,
    matcher_url_path_op: Option<&RuleOp>,
) -> bool {
    let matcher_url_path_op = if let Some(url_path_op) = matcher_url_path_op {
        url_path_op
    } else {
        &RuleOp::Equal
    };
    if !matcher_url_path_op.is_match(request_uri_path, matcher_url_path) {
        return false;
    }
    true
}

/// check if `headers` in `when` matches
pub fn headers_is_match(
    request_headers: &HeaderMap<HeaderValue>,
    matcher_headers: &HashMap<String, ConditionStatement>,
    rule_idx: usize,
    rule_set_idx: usize,
) -> bool {
    if matcher_headers
        .iter()
        .any(|(matcher_header_key, matcher_header_value)| {
            let request_header_value = request_headers.get(matcher_header_key);
            if request_header_value.is_none() {
                return true;
            }
            let request_header_value = match request_header_value.unwrap().to_str() {
                Ok(x) => x,
                Err(err) => {
                    log::error!(
                        "failed to get value from {} in rule {} in rule set {}\n({})",
                        matcher_header_key,
                        rule_idx,
                        rule_set_idx,
                        err
                    );
                    return true;
                }
            };
            !matcher_header_value
                .op
                .is_match(request_header_value, &matcher_header_value.value)
        })
    {
        return false;
    }
    true
}

/// check if `body` in `when` matches
pub fn body_is_match(
    request: &ParsedRequest,
    matcher_body: &HashMap<BodyKind, BodyCondition>,
    rule_idx: usize,
    rule_set_idx: usize,
) -> bool {
    let request_content_type = match request.component_parts.headers.get(CONTENT_TYPE) {
        Some(x) => Ok(x),
        None => Err(()),
    };
    if request_content_type.is_err() {
        log::error!(
            "failed to get content-type of request (met in rule {} in rule set {})",
            rule_idx,
            rule_set_idx
        );
        return false;
    }
    let request_content_type = request_content_type.unwrap();

    if !content_type_is_application_json(request_content_type) {
        return false;
    }

    let request_body_json = request.body_json.clone();
    if request_body_json.is_none() {
        return false;
    }
    let request_body_json = request_body_json.unwrap();

    let matcher_body_json = matcher_body.get(&BodyKind::Json);
    if 0 < matcher_body.len() && matcher_body_json.is_none() {
        return false;
    }
    let matcher_body_json = matcher_body_json.unwrap();

    if let Some(matcher_json_condition) = &matcher_body_json.json_condition {
        if matcher_json_condition.iter().any(
            |(matcher_json_condition_key, matcher_json_condition_statement)| {
                let request_body_json_value = request_body_json.get(matcher_json_condition_key);
                if request_body_json_value.is_none() {
                    return true;
                }
                let request_body_json_value = request_body_json_value.unwrap();

                matcher_json_condition_statement.op.is_match(
                    request_body_json_value.to_string().as_str(),
                    &matcher_json_condition_statement.value,
                )
            },
        ) {
            return false;
        }
    }

    true
}
