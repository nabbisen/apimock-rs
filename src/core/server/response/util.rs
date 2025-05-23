use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    http::response::Builder,
    StatusCode,
};
use serde_json::{Map, Value};

use std::{collections::HashMap, path::Path};

use super::{default_builder, json_builder};
use crate::core::server::constant::DEFAULT_PLAIN_TEXT_CONTENT_TYPE;

/// file extension string from file path
pub fn file_extension(file_path: &str) -> Option<String> {
    match Path::new(file_path)
        .extension()
        .unwrap_or_default()
        .to_ascii_lowercase()
        .to_str()
    {
        Some(x) => Some(x.to_owned()),
        None => None,
    }
}

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
            .fold(builder, |builder, (header_key, header_value)| {
                builder.header(header_key, header_value.clone().unwrap_or_default())
            });
    }

    builder
}

/// generate response builder for binary file
pub fn binary_content_builder(
    file_path: &str,
    custom_headers: Option<&HashMap<String, Option<String>>>,
) -> Builder {
    let mut builder = default_builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, binary_content_type(file_path));

    if let Some(headers) = custom_headers {
        builder = headers
            .iter()
            .fold(builder, |builder, (header_key, header_value)| {
                builder.header(header_key, header_value.clone().unwrap_or_default())
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

fn binary_content_type(file_path: &str) -> HeaderValue {
    let content_type = match file_extension(file_path).unwrap_or_default().as_str() {
        // - image
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "bmp" => "image/bmp",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",

        // - sound
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "flac" => "audio/flac",
        "ogg" => "audio/ogg",
        "m4a" => "audio/mp4",
        "aac" => "audio/aac",
        "weba" => "audio/webm",

        // video
        "mp4" => "video/mp4",
        "m4v" => "video/mp4",
        "mpeg" | "mpg" => "video/mpeg",
        "avi" => "video/x-msvideo",
        "mov" => "video/quicktime",
        "webm" => "video/webm",
        "ogv" => "video/ogg",

        // - doc
        "pdf" => "application/pdf",
        // - archive
        "zip" => "application/zip",
        // - (else)
        _ => "application/octet-stream",
    };
    HeaderValue::from_static(content_type)
}
