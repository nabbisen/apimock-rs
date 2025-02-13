use serde_json::Value;

/// get value from json value with jsonpath
pub fn jsonpath_value(json: &Value, jsonpath: &str) -> Option<Value> {
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
