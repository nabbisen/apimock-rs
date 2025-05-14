use http_body_util::{BodyExt, Full};
use hyper::{
    body::Bytes,
    header::{HeaderValue, CONTENT_TYPE},
    http::Error,
    StatusCode,
};

use super::default;
use crate::core::server::{constant::DEFAULT_PLAIN_TEXT_CONTENT_TYPE, types::BoxBody};

/// plain text response
pub fn plain_text(
    content: &str,
    content_type: Option<&str>,
) -> Result<hyper::Response<BoxBody>, Error> {
    let default_content_type = HeaderValue::from_static(DEFAULT_PLAIN_TEXT_CONTENT_TYPE);
    let response_content_type = if let Some(content_type) = content_type {
        HeaderValue::from_str(content_type).unwrap_or_else(|_| default_content_type)
    } else {
        default_content_type
    };

    default()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, response_content_type)
        .body(Full::new(Bytes::from(content.to_owned())).boxed())
}
