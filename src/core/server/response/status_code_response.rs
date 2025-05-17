use http_body_util::{BodyExt, Empty, Full};
use hyper::{body::Bytes, StatusCode};

use crate::core::server::types::BoxBody;

use super::default_builder;

/// custom status code response (body is empty)
pub fn status_code_response(
    status_code: &StatusCode,
) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
    default_builder()
        .status(status_code)
        .body(Empty::new().boxed())
}

/// custom status code response with message in body
pub fn status_code_response_with_message(
    status_code: &StatusCode,
    message: &str,
) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
    default_builder()
        .status(status_code)
        .body(Full::new(Bytes::from(message.to_owned())).boxed())
}
