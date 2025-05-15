use http_body_util::{BodyExt, Full};
use hyper::{
    body::Bytes,
    header::{HeaderValue, CONTENT_TYPE},
    http::response::Builder,
    StatusCode,
};
use serde_json::{json, Map, Value};

use std::{collections::HashMap, path::Path};

use crate::core::server::{constant::CSV_RECORDS_DEFAULT_KEY, types::BoxBody};

use super::{
    default_builder,
    error_response::{bad_request_response, internal_server_error_response},
    json_builder,
    text_response::text_response,
    util::text_file_content_type,
};

pub struct FileResponse {
    file_path: String,
    text_content: Option<String>,
    binary_content: Option<Vec<u8>>,
    custom_headers: Option<HashMap<String, Option<String>>>,
}

impl FileResponse {
    /// create instance
    pub fn new(file_path: &str, custom_headers: Option<&HashMap<String, Option<String>>>) -> Self {
        FileResponse {
            file_path: file_path.to_owned(),
            text_content: None,
            binary_content: None,
            custom_headers: custom_headers.cloned(),
        }
    }

    /// response from file path
    pub fn file_content_response(
        &mut self,
    ) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
        if Path::new(self.file_path.as_str()).is_dir() {
            return bad_request_response(format!("{} is directory", self.file_path).as_str());
        }

        match std::fs::read_to_string(self.file_path.as_str()) {
            Ok(content) => {
                self.text_content = Some(content);
                self.text_file_content_response()
            }
            Err(_) => match std::fs::read(self.file_path.as_str()) {
                Ok(content) => {
                    self.binary_content = Some(content);
                    self.binary_file_content_response()
                }
                Err(err) => internal_server_error_response(
                    format!("{}: failed to read file - {}", self.file_path, err).as_str(),
                ),
            },
        }
    }

    /// text file response
    fn text_file_content_response(&self) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
        match Path::new(self.file_path.as_str())
            .extension()
            .unwrap_or_default()
            .to_ascii_lowercase()
            .to_str()
        {
            Some(ext) => match ext {
                "json" | "json5" => self.json_file_content_response(),
                "csv" => self.csv_file_content_response(),
                _ => text_response(
                    self.text_content.clone().unwrap_or_default().as_str(),
                    Some(text_file_content_type(ext).as_str()),
                    None,
                ),
            },
            None => text_response(
                self.text_content.clone().unwrap_or_default().as_str(),
                None,
                None,
            ),
        }
    }

    /// json file response
    fn json_file_content_response(&self) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
        match json5::from_str::<Value>(self.text_content.clone().unwrap_or_default().as_str()) {
            Ok(content) => {
                let body = content.to_string();
                json_content_builder(self.custom_headers.as_ref())
                    .body(Full::new(Bytes::from(body)).boxed())
            }
            _ => internal_server_error_response(
                format!("{}: Invalid json content", self.file_path.as_str()).as_str(),
            ),
        }
    }

    /// csv file response
    fn csv_file_content_response(&self) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
        let text_content = self.text_content.clone().unwrap_or_default();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(text_content.as_bytes());

        let csv_headers = if let Ok(csv_headers) = rdr.headers() {
            csv_headers.clone()
        } else {
            return internal_server_error_response(
                format!("{}: failed to analyze csv headers", self.file_path.as_str()).as_str(),
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
                    Ok(body) => json_content_builder(self.custom_headers.as_ref())
                        .body(Full::new(Bytes::from(body)).boxed()),
                    Err(err) => internal_server_error_response(
                        format!(
                            "{}: failed to convert csv records to json response - {}",
                            self.file_path.as_str(),
                            err
                        )
                        .as_str(),
                    ),
                }
            }
            Err(err) => internal_server_error_response(
                format!(
                    "{}: failed to analyze csv records - {}",
                    self.file_path.as_str(),
                    err
                )
                .as_str(),
            ),
        }
    }

    /// binary file response
    fn binary_file_content_response(&self) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
        let content = self.binary_content.clone().unwrap_or_default().to_owned();
        binary_content_builder(self.custom_headers.as_ref())
            .body(Full::new(Bytes::from(content)).boxed())
    }
}

/// generate response builder for json compatible file
fn json_content_builder(custom_headers: Option<&HashMap<String, Option<String>>>) -> Builder {
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
fn binary_content_builder(custom_headers: Option<&HashMap<String, Option<String>>>) -> Builder {
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
