use http_body_util::{BodyExt, Empty, Full};
use hyper::{
    body::Bytes,
    header::{
        HeaderValue, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
        ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE,
    },
    http::{response::Builder, Error},
    Response, StatusCode,
};
use serde_json::{json, Map, Value};

use std::{collections::HashMap, path::Path};

use super::{constant::DEFAULT_PLAIN_TEXT_CONTENT_TYPE, types::BoxBody};
use crate::core::{
    config::{HeaderConfig, HeaderId, PathConfig},
    server::constant::CSV_RECORDS_DEFAULT_KEY,
};

/// response on `data_dir` paths
pub fn static_path_response(
    path_config: &PathConfig,
    headers: Option<&HashMap<HeaderId, HeaderConfig>>,
    request_uri_path: &str,
) -> Result<Response<BoxBody>, Error> {
    if let Some(_) = &path_config.data_src {
        return static_path_data_src_reponse(path_config, headers, request_uri_path);
    }

    if let Some(data_text) = &path_config.data_text {
        return json_response_base(path_config.headers.as_ref(), headers)
            .status(path_config.code)
            .body(Full::new(Bytes::from(data_text.to_owned())).boxed());
    }

    response_base(path_config.headers.as_ref(), headers)
        .status(path_config.code)
        .body(Empty::new().boxed())
}

/// json file on `data_src` response on `data_dir` paths
fn static_path_data_src_reponse(
    path_config: &PathConfig,
    headers: Option<&HashMap<HeaderId, HeaderConfig>>,
    request_uri_path: &str,
) -> Result<Response<BoxBody>, Error> {
    match path_config.data_src.as_ref() {
        Some(data_src) => {
            file_to_response(data_src.as_str(), path_config.headers.as_ref(), headers)
        }
        None => internal_server_error_response(
            format!("{}: data_src is missing", request_uri_path).as_str(),
        ),
    }
}

/// response from file path
pub fn file_to_response(
    filepath: &str,
    path_headers: Option<&Vec<String>>,
    headers: Option<&HashMap<HeaderId, HeaderConfig>>,
) -> Result<Response<BoxBody>, Error> {
    if Path::new(filepath).is_dir() {
        return bad_request_response(format!("{} is directory", filepath).as_str());
    }

    match std::fs::read_to_string(filepath) {
        Ok(content) => text_file_to_response(content.as_str(), filepath, path_headers, headers),
        Err(_) => match std::fs::read(filepath) {
            Ok(content) => binary_file_to_response(content.as_ref()),
            Err(err) => internal_server_error_response(
                format!("{}: failed to read file - {}", filepath, err).as_str(),
            ),
        },
    }
}

/// text file response
fn text_file_to_response(
    content: &str,
    filepath: &str,
    path_headers: Option<&Vec<String>>,
    headers: Option<&HashMap<HeaderId, HeaderConfig>>,
) -> Result<Response<BoxBody>, Error> {
    match Path::new(filepath)
        .extension()
        .unwrap_or_default()
        .to_ascii_lowercase()
        .to_str()
    {
        Some(ext) => match ext {
            "json" | "json5" => {
                json_text_file_to_response(content, filepath, path_headers, headers)
            }
            "csv" => csv_text_file_to_response(content, filepath, path_headers, headers),
            _ => plain_text_response(content, Some(text_file_content_type(ext).as_str())),
        },
        None => plain_text_response(content, None),
    }
}

/// json file response
fn json_text_file_to_response(
    content: &str,
    filepath: &str,
    path_headers: Option<&Vec<String>>,
    headers: Option<&HashMap<HeaderId, HeaderConfig>>,
) -> Result<Response<BoxBody>, Error> {
    match json5::from_str::<Value>(content) {
        Ok(json) => {
            let body = json.to_string();
            json_response_base(path_headers, headers)
                .status(StatusCode::OK)
                .body(Full::new(Bytes::from(body)).boxed())
        }
        _ => internal_server_error_response(format!("{}: Invalid json content", filepath).as_str()),
    }
}

/// csv file response
fn csv_text_file_to_response(
    content: &str,
    filepath: &str,
    path_headers: Option<&Vec<String>>,
    headers: Option<&HashMap<HeaderId, HeaderConfig>>,
) -> Result<Response<BoxBody>, Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(content.as_bytes());

    let csv_headers = if let Ok(csv_headers) = rdr.headers() {
        csv_headers.clone()
    } else {
        return internal_server_error_response(
            format!("{}: failed to analyze csv headers", filepath).as_str(),
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
                Ok(body) => json_response_base(path_headers, headers)
                    .status(StatusCode::OK)
                    .body(Full::new(Bytes::from(body)).boxed()),
                Err(err) => internal_server_error_response(
                    format!(
                        "{}: failed to convert csv records to json response - {}",
                        filepath, err
                    )
                    .as_str(),
                ),
            }
        }
        Err(err) => internal_server_error_response(
            format!("{}: failed to analyze csv records - {}", filepath, err).as_str(),
        ),
    }
}

/// binary file response
fn binary_file_to_response(content: &Vec<u8>) -> Result<Response<BoxBody>, Error> {
    let content = content.to_owned();
    response_base(None, None)
        .status(StatusCode::OK)
        .header(
            CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        )
        .body(Full::new(Bytes::from(content)).boxed())
}

/// response base on any
pub fn response_base(
    path_headers: Option<&Vec<String>>,
    headers: Option<&HashMap<HeaderId, HeaderConfig>>,
) -> Builder {
    let mut ret = Response::builder()
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"))
        .header(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"))
        .header(
            ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("GET, POST, OPTIONS"),
        );
    if let Some(path_headers) = path_headers {
        let headers = headers.clone().unwrap();
        for path_header in path_headers {
            let header = headers.get(path_header).unwrap();
            ret = ret.header(header.key.as_str(), header.value.clone().unwrap().as_str());
        }
    }
    ret
}

/// response base on json response
pub fn json_response_base(
    path_headers: Option<&Vec<String>>,
    headers: Option<&HashMap<HeaderId, HeaderConfig>>,
) -> Builder {
    response_base(path_headers, headers)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
}

/// plain text response
pub fn plain_text_response(
    content: &str,
    content_type: Option<&str>,
) -> Result<Response<BoxBody>, Error> {
    let default_content_type = HeaderValue::from_static(DEFAULT_PLAIN_TEXT_CONTENT_TYPE);
    let response_content_type = if let Some(content_type) = content_type {
        HeaderValue::from_str(content_type).unwrap_or_else(|_| default_content_type)
    } else {
        default_content_type
    };

    response_base(None, None)
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, response_content_type)
        .body(Full::new(Bytes::from(content.to_owned())).boxed())
}

/// error response on http BAD_REQUEST (400)
pub fn bad_request_response(msg: &str) -> Result<Response<BoxBody>, Error> {
    response_base(None, None)
        .status(StatusCode::BAD_REQUEST)
        .body(Full::new(Bytes::from(msg.to_owned())).boxed())
}

/// error response on http NOT_FOUND (404)
pub fn not_found_response() -> Result<Response<BoxBody>, Error> {
    response_base(None, None)
        .status(StatusCode::NOT_FOUND)
        .body(Empty::new().boxed())
}

/// error response on http INTERNAL_SERVER_ERROR (500)
pub fn internal_server_error_response(msg: &str) -> Result<Response<BoxBody>, Error> {
    response_base(None, None)
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Full::new(Bytes::from(msg.to_owned())).boxed())
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
