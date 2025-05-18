use body_kind::BodyKind;
use hyper::header::CONTENT_TYPE;
use serde::Deserialize;

use std::collections::HashMap;

mod body_kind;

use crate::core::{
    server::{
        parsed_request::ParsedRequest,
        routing::rule_set::rule::{when::condition_statement::ConditionStatement, ConditionKey},
    },
    util::{http::content_type_is_application_json, json::json_value_by_jsonpath},
};

#[derive(Clone, Debug, Deserialize)]
#[serde(transparent)]
pub struct Body(pub HashMap<BodyKind, HashMap<ConditionKey, ConditionStatement>>);

impl Body {
    /// check if `body` in `when` matches
    pub fn is_match(
        &self,
        sent_request: &ParsedRequest,
        rule_idx: usize,
        rule_set_idx: usize,
    ) -> bool {
        let request_content_type = match sent_request.component_parts.headers.get(CONTENT_TYPE) {
            Some(x) => Ok(x),
            None => Err(()),
        };
        if request_content_type.is_err() {
            log::error!(
                "failed to get content-type of request (rule #{} in rule set #{})",
                rule_idx + 1,
                rule_set_idx + 1
            );
            return false;
        }
        let request_content_type = request_content_type.unwrap();

        if !content_type_is_application_json(request_content_type) {
            return false;
        }

        let request_body_json = sent_request.body_json.clone();
        if request_body_json.is_none() {
            return false;
        }
        let request_body_json = request_body_json.unwrap();

        // todo: support other types than json (such as form value) in the future
        let matcher_body_json_condition = match self.0.get(&BodyKind::Json) {
            Some(x) if 0 < x.len() => x,
            _ => return false,
        };

        if matcher_body_json_condition.iter().any(
            |(matcher_json_condition_key, matcher_json_condition_statement)| {
                let request_body_json_value =
                    json_value_by_jsonpath(&request_body_json, matcher_json_condition_key);

                if request_body_json_value.is_none() {
                    return false;
                }

                let request_body_json_value = request_body_json_value.unwrap();
                match request_body_json_value.as_str() {
                    Some(request_body_json_value) => matcher_json_condition_statement
                        .op
                        .clone()
                        .unwrap_or_default()
                        .is_match(
                            request_body_json_value,
                            &matcher_json_condition_statement.value,
                        ),
                    None => false,
                }
            },
        ) {
            return true;
        }

        false
    }

    /// validate
    pub fn validate(&self) -> bool {
        if self.0.len() == 0 {
            return false;
        }

        for (_, body_kind_map) in self.0.iter() {
            if body_kind_map.len() == 0 {
                return false;
            }
        }

        true
    }
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (body_kind, body_condition) in self.0.iter() {
            let body_condition_fmt = body_condition
                .iter()
                .map(|(key, statement)| format!("{}{}", key, statement))
                .collect::<Vec<String>>()
                .join(", ");

            let _ = write!(f, "[{}] {}", body_kind, body_condition_fmt);
        }

        Ok(())
    }
}
