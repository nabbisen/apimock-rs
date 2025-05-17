use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    http::response::Builder,
    StatusCode,
};
use serde_json::{Map, Value};

use std::collections::HashMap;

use super::{default_builder, json_builder};
use crate::core::server::constant::DEFAULT_PLAIN_TEXT_CONTENT_TYPE;

/// content type from text file extension
pub fn text_file_content_type(ext: &str) -> String {
    let ret = match ext {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        _ => DEFAULT_PLAIN_TEXT_CONTENT_TYPE,
    };
    ret.to_owned()
}

/// generate response builder for json compatible file
pub fn json_content_builder(custom_headers: Option<&HashMap<String, Option<String>>>) -> Builder {
    let mut builder = json_builder().status(StatusCode::OK);

    if let Some(headers) = custom_headers {
        builder = headers
            .iter()
            .fold(builder, |builder, (header_name, header_value)| {
                builder.header(header_name, header_value.clone().unwrap_or_default())
            });
    }

    builder
}

/// generate response builder for binary file
pub fn binary_content_builder(custom_headers: Option<&HashMap<String, Option<String>>>) -> Builder {
    let mut builder = default_builder().status(StatusCode::OK).header(
        CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );

    if let Some(headers) = custom_headers {
        builder = headers
            .iter()
            .fold(builder, |builder, (header_name, header_value)| {
                builder.header(header_name, header_value.clone().unwrap_or_default())
            });
    }

    builder
}

/// json value with jsonpath as key
pub fn json_value_with_jsonpath_key(jsonpath_key: &str, value: Value) -> Value {
    let mut keys: Vec<&str> = jsonpath_key.split('.').collect();
    keys.reverse();

    let mut ret = value;

    for key in keys {
        let mut map = Map::new();
        map.insert(key.to_string(), ret);
        ret = Value::Object(map);
    }

    ret
}
