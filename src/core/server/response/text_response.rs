use std::collections::HashMap;

use http_body_util::{BodyExt, Full};
use hyper::{
    body::Bytes,
    header::{HeaderValue, CONTENT_TYPE},
    http::response::Builder,
    StatusCode,
};

use super::default_builder;
use crate::core::server::{constant::DEFAULT_PLAIN_TEXT_CONTENT_TYPE, types::BoxBody};

/// plain text response
pub fn text_response(
    content: &str,
    content_type: Option<&str>,
    custom_headers: Option<&HashMap<String, Option<String>>>,
) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
    let mut builder = builder(content_type);

    if let Some(headers) = custom_headers {
        builder = headers
            .iter()
            .fold(builder, |builder, (header_key, header_value)| {
                builder.header(header_key, header_value.clone().unwrap_or_default())
            });
    }

    builder.body(Full::new(Bytes::from(content.to_owned())).boxed())
}

/// generate builder
fn builder(content_type: Option<&str>) -> Builder {
    let default_content_type = HeaderValue::from_static(DEFAULT_PLAIN_TEXT_CONTENT_TYPE);
    let response_content_type = if let Some(content_type) = content_type {
        HeaderValue::from_str(content_type).unwrap_or_else(|_| default_content_type)
    } else {
        default_content_type
    };

    default_builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, response_content_type)
}
