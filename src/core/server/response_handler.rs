use http_body_util::{BodyExt, Empty, Full};
use hyper::{
    body::Bytes,
    header::{HeaderName, HeaderValue, ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_LENGTH, ORIGIN, VARY},
    http::response::Builder,
    HeaderMap, StatusCode,
};

use std::{collections::HashMap, str::FromStr};

use super::constant::DEFAULT_RESPONSE_HEADERS;
use crate::core::server::types::BoxBody;

#[derive(Clone)]
pub enum BodyKind {
    Empty,
    Text(String),
    Binary(Vec<u8>),
}

impl Default for BodyKind {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Default)]
pub struct ResponseHandler {
    response_builder: Builder,
    status: Option<StatusCode>,
    headers: HashMap<String, Option<String>>,
    body_kind: BodyKind,
}

impl ResponseHandler {
    /// build response
    pub fn into_response(
        mut self,
        request_headers: &HeaderMap,
    ) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
        // - default headers but content-length (later)
        self.response_builder = default_response_headers(request_headers).into_iter().fold(
            self.response_builder,
            |builder, (header_key, header_value)| {
                if let Some(header_key) = header_key {
                    builder.header(header_key, header_value)
                } else {
                    builder
                }
            },
        );

        // - additional custom headers passed from caller
        self.response_builder =
            self.headers
                .iter()
                .fold(
                    self.response_builder,
                    |response_builder, (header_key, header_value)| match HeaderName::from_str(header_key) {
                        Ok(header_key) => {
                            match HeaderValue::from_str(
                                header_value.clone().unwrap_or_default().as_str(),
                            ) {
                                Ok(header_value) => response_builder.header(header_key, header_value),
                                Err(err) => {
                                    log::warn!(
                                        "only header key set because failed to get header value: {} [key = {}] ({})",
                                        header_value.clone().unwrap_or_default(),
                                        header_key,
                                        err
                                    );
                                    response_builder.header(header_key, HeaderValue::from_static(""))
                                }
                            }
                        }
                        Err(err) => {
                            log::warn!("failed to set header key: {} ({})", header_key, err);
                            response_builder
                        }
                    },
                );

        // - http status code
        let status = if let Some(status) = self.status {
            status
        } else {
            StatusCode::OK
        };
        self.response_builder = self.response_builder.status(status);

        // - body + content-length
        let ret = match self.body_kind {
            BodyKind::Text(s) => self
                .response_builder
                .header(CONTENT_LENGTH, s.as_bytes().len())
                .body(Full::new(Bytes::from(s.to_owned())).boxed()),
            BodyKind::Binary(b) => self
                .response_builder
                .header(CONTENT_LENGTH, b.len())
                .body(Full::new(Bytes::from(b)).boxed()),
            BodyKind::Empty => self.response_builder.body(Empty::new().boxed()),
        };

        ret
    }

    /// set http status code
    pub fn with_status(mut self, status: &StatusCode) -> Self {
        self.status = Some(status.to_owned());
        self
    }

    /// add custom header
    pub fn with_header(mut self, key: impl Into<String>, value: Option<impl Into<String>>) -> Self {
        self.headers.insert(key.into(), value.map(|x| x.into()));
        self
    }

    /// add custom headers
    pub fn with_headers<K, V, I>(mut self, headers: I) -> Self
    where
        K: Into<String>,
        V: Into<String>,
        I: IntoIterator<Item = (K, Option<V>)>,
    {
        for (key, value) in headers {
            self.headers.insert(key.into(), value.map(|x| x.into()));
        }
        self
    }

    /// add text to body
    pub fn with_text(mut self, text: impl Into<String>, content_type: Option<&str>) -> Self {
        let content_type = if let Some(content_type) = content_type {
            content_type.into()
        } else {
            "text/plain; charset=utf-8".to_owned()
        };
        self.headers
            .insert("content-type".into(), Some(content_type));

        self.body_kind = BodyKind::Text(text.into());
        self
    }

    /// treat response as json
    pub fn with_json_body(mut self, body: impl Into<String>) -> Self {
        self.headers
            .insert("content-type".into(), Some("application/json".into()));
        self.body_kind = BodyKind::Text(body.into());
        self
    }

    /// treat response as json
    pub fn with_binary_body(
        mut self,
        body: Vec<u8>,
        content_type: Option<impl Into<String>>,
    ) -> Self {
        let content_type = if let Some(content_type) = content_type {
            content_type.into()
        } else {
            "application/octet-stream".to_owned()
        };
        self.headers
            .insert("content-type".into(), Some(content_type));

        self.body_kind = BodyKind::Binary(body);

        self
    }
}

/// default response headers key-value pairs
pub fn default_response_headers(request_headers: &HeaderMap) -> HeaderMap {
    let mut header_map_src = Vec::with_capacity(DEFAULT_RESPONSE_HEADERS.len() + 1);

    // resource
    // - the other default headers but access-control-allow-origin, vary
    header_map_src.extend(
        DEFAULT_RESPONSE_HEADERS
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string())),
    );

    // - access-control-allow-origin, vary
    let (origin, vary) = match request_headers.get(ORIGIN) {
        Some(x) => (x.to_owned(), HeaderValue::from_static("Origin")),
        None => (HeaderValue::from_static("*"), HeaderValue::from_static("*")),
    };
    header_map_src.push((
        ACCESS_CONTROL_ALLOW_ORIGIN.to_string(),
        origin.to_str().unwrap_or_default().to_owned(),
    ));
    header_map_src.push((
        VARY.to_string(),
        vary.to_str().unwrap_or_default().to_owned(),
    ));

    // header map
    let ret = header_map_src.iter().fold(HeaderMap::new(),|mut ret,(header_key, header_value)| {
        match HeaderName::from_str(header_key) {
            Ok(header_key) => {
                match HeaderValue::from_str(
                    header_value.as_str(),
                ) {
                    Ok(header_value) => {
                        ret.insert(header_key, header_value);
                        ret
                    },
                    Err(err) => {
                        log::warn!(
                            "only header key set because failed to get header value: {} [key = {}] ({})",
                            header_value.as_str(),
                            header_key,
                            err
                        );
                        ret.insert(header_key, HeaderValue::from_static(""));
                        ret
                    }
                }
            }
            Err(err) => {
                log::warn!("failed to set header key: {} ({})", header_key, err);
                ret
            }
    }});

    ret
}
