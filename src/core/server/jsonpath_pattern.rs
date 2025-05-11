use serde_json::Value;

use std::collections::HashMap;

use crate::core::config::{JsonpathMatchingPattern, PathConfig};

/// get value from json value with jsonpath
fn jsonpath_value(json: &Value, jsonpath: &str) -> Option<Value> {
    let mut jsonpath_parts = jsonpath.split('.');
    let ret = jsonpath_value_recursive(json, &mut jsonpath_parts);
    ret
}

/// recursively scan json due to jsonpath
fn jsonpath_value_recursive<'a>(
    value: &'a Value,
    jsonpath_parts: &mut std::str::Split<'_, char>,
) -> Option<Value> {
    if let Some(current_part) = jsonpath_parts.next() {
        match value {
            Value::Object(map) => {
                if let Some(value) = map.get(current_part) {
                    jsonpath_value_recursive(value, jsonpath_parts)
                } else {
                    None
                }
            }
            Value::Array(arr) => {
                if let Ok(index) = current_part.parse::<usize>() {
                    if let Some(item) = arr.get(index) {
                        jsonpath_value_recursive(item, jsonpath_parts)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Value::Number(_) => Some(value.to_owned()),
            Value::String(_) => Some(value.to_owned()),
            _ => None,
        }
    } else {
        Some(value.clone())
    }
}

/// update path config if matcher finds pair
pub async fn match_jsonpath_patterns(
    path_config: &mut PathConfig,
    request_uri_path: &str,
    request_body_json_value: &Value,
    paths_jsonpath_patterns: Option<
        &HashMap<String, HashMap<String, Vec<JsonpathMatchingPattern>>>,
    >,
) {
    if let Some(paths_jsonpath_patterns) = paths_jsonpath_patterns {
        if let Some(key) = paths_jsonpath_patterns
            .keys()
            .find(|x| x.as_str() == request_uri_path)
        {
            let jsonpath_patterns = paths_jsonpath_patterns.get(key).unwrap();
            for jsonpath in jsonpath_patterns.keys() {
                if let Some(value) = jsonpath_value(request_body_json_value, jsonpath) {
                    let request_json_value = match value {
                        Value::String(x) => x,
                        Value::Number(x) => x.to_string(),
                        _ => {
                            continue;
                        }
                    };
                    let patterns = jsonpath_patterns.get(jsonpath).unwrap();
                    if let Some(matched) = patterns
                        .iter()
                        .find(|x| x.value.as_str() == request_json_value.as_str())
                    {
                        // first matched only
                        path_config.data_src = Some(matched.data_src.to_owned());
                        break;
                    }
                }
            }
        }
    }
}
