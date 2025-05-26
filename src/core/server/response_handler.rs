use http_body_util::{BodyExt, Empty, Full};
use hyper::{
    body::Bytes,
    header::{HeaderName, HeaderValue, ACCESS_CONTROL_ALLOW_ORIGIN, ORIGIN},
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
        // - access-control-allow-origin (one of the default headers)
        let fallback_origin = HeaderValue::from_static("*");
        let client_origin = request_headers
            .get(ORIGIN)
            .unwrap_or_else(|| &fallback_origin);
        self.response_builder = self
            .response_builder
            .header(ACCESS_CONTROL_ALLOW_ORIGIN, client_origin);

        // - the other default headers
        self.response_builder = DEFAULT_RESPONSE_HEADERS.iter().fold(
            self.response_builder,
            |builder, (header_key, header_value)| {
                builder.header(
                    HeaderName::from_static(header_key),
                    HeaderValue::from_static(header_value),
                )
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

        // - body
        let ret = match self.body_kind {
            BodyKind::Text(s) => self
                .response_builder
                .body(Full::new(Bytes::from(s.to_owned())).boxed()),
            BodyKind::Binary(b) => self
                .response_builder
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
