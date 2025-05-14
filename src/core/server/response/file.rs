use http_body_util::{BodyExt, Full};
use hyper::{
    body::Bytes,
    header::{HeaderValue, CONTENT_TYPE},
    http::Error,
    StatusCode,
};
use serde_json::{Map, Value};

use std::path::Path;

use crate::core::server::{constant::CSV_RECORDS_DEFAULT_KEY, types::BoxBody};

use super::{
    default,
    error::{bad_request, internal_server_error},
    json,
    text::plain_text,
    util::text_file_content_type,
};

/// response from file path
pub fn file_content(file_path: &str) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
    if Path::new(file_path).is_dir() {
        return bad_request(format!("{} is directory", file_path).as_str());
    }

    match std::fs::read_to_string(file_path) {
        Ok(content) => text_file_content(content.as_str(), file_path),
        Err(_) => match std::fs::read(file_path) {
            Ok(content) => binary_file_content(content.as_ref()),
            Err(err) => internal_server_error(
                format!("{}: failed to read file - {}", file_path, err).as_str(),
            ),
        },
    }
}

/// text file response
fn text_file_content(
    content: &str,
    file_path: &str,
) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
    match Path::new(file_path)
        .extension()
        .unwrap_or_default()
        .to_ascii_lowercase()
        .to_str()
    {
        Some(ext) => match ext {
            "json" | "json5" => json_file_content(content, file_path),
            "csv" => csv_file_content(content, file_path),
            _ => plain_text(content, Some(text_file_content_type(ext).as_str())),
        },
        None => plain_text(content, None),
    }
}

/// json file response
fn json_file_content(
    content: &str,
    file_path: &str,
) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
    match json5::from_str::<Value>(content) {
        Ok(content) => {
            let body = content.to_string();
            json()
                .status(StatusCode::OK)
                .body(Full::new(Bytes::from(body)).boxed())
        }
        _ => internal_server_error(format!("{}: Invalid json content", file_path).as_str()),
    }
}

/// csv file response
fn csv_file_content(
    content: &str,
    file_path: &str,
) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(content.as_bytes());

    let csv_headers = if let Ok(csv_headers) = rdr.headers() {
        csv_headers.clone()
    } else {
        return internal_server_error(
            format!("{}: failed to analyze csv headers", file_path).as_str(),
        );
    };

    let rows = rdr
        .records()
        .map(|result| {
            let record = result?;
            let obj = csv_headers
                .iter()
                .zip(record.iter())
                .map(|(k, v)| (k.to_string(), Value::String(v.to_string())))
                .collect::<Map<_, _>>();
            Ok(Value::Object(obj))
        })
        .collect::<Result<Vec<Value>, csv::Error>>();

    match rows {
        Ok(rows) => {
            let json_value = json!({ CSV_RECORDS_DEFAULT_KEY: &rows });
            let body = serde_json::to_string(&json_value);
            match body {
                Ok(body) => json()
                    .status(StatusCode::OK)
                    .body(Full::new(Bytes::from(body)).boxed()),
                Err(err) => internal_server_error(
                    format!(
                        "{}: failed to convert csv records to json response - {}",
                        file_path, err
                    )
                    .as_str(),
                ),
            }
        }
        Err(err) => internal_server_error(
            format!("{}: failed to analyze csv records - {}", file_path, err).as_str(),
        ),
    }
}

/// binary file response
fn binary_file_content(content: &Vec<u8>) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
    let content = content.to_owned();
    default()
        .status(StatusCode::OK)
        .header(
            CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        )
        .body(Full::new(Bytes::from(content)).boxed())
}
