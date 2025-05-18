use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use serde_json::{Map, Value};

use std::{collections::HashMap, path::Path};

use crate::core::{
    server::{constant::CSV_RECORDS_DEFAULT_KEY, types::BoxBody},
    util::json::resolve_with_json_compatible_extensions,
};

use super::{
    error_response::{bad_request_response, internal_server_error_response},
    text_response::text_response,
    util::{
        binary_content_builder, json_content_builder, json_value_with_jsonpath_key,
        text_file_content_type,
    },
};

pub struct FileResponse {
    file_path: String,
    csv_records_key: Option<String>,
    text_content: Option<String>,
    binary_content: Option<Vec<u8>>,
    custom_headers: Option<HashMap<String, Option<String>>>,
}

impl FileResponse {
    /// create instance
    pub fn new(file_path: &str, custom_headers: Option<&HashMap<String, Option<String>>>) -> Self {
        FileResponse {
            file_path: file_path.to_owned(),
            csv_records_key: None,
            text_content: None,
            binary_content: None,
            custom_headers: custom_headers.cloned(),
        }
    }

    /// create instance
    pub fn new_with_csv_records_jsonpath(
        file_path: &str,
        custom_headers: Option<&HashMap<String, Option<String>>>,
        csv_records_key: Option<String>,
    ) -> Self {
        let mut ret = FileResponse::new(file_path, custom_headers);
        ret.csv_records_key = csv_records_key;
        ret
    }

    /// response from file path
    pub fn file_content_response(
        &mut self,
    ) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
        let file_path = match resolve_with_json_compatible_extensions(self.file_path.as_str()) {
            Some(x) => x,
            None => {
                return bad_request_response(format!("{} is directory", self.file_path).as_str())
            }
        };
        self.file_path = file_path;

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
                let jsonpath_key = if let Some(csv_records_key) = self.csv_records_key.as_ref() {
                    csv_records_key.as_str()
                } else {
                    CSV_RECORDS_DEFAULT_KEY
                };
                let json_value = json_value_with_jsonpath_key(jsonpath_key, Value::from(rows));

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
