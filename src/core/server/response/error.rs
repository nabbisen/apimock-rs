use http_body_util::{BodyExt, Empty, Full};
use hyper::{body::Bytes, StatusCode};

use super::default_builder;
use crate::core::server::types::BoxBody;

/// error response on http BAD_REQUEST (400)
pub fn bad_request_response(msg: &str) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
    default_builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Full::new(Bytes::from(msg.to_owned())).boxed())
}

/// error response on http NOT_FOUND (404)
pub fn not_found_response() -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
    default_builder()
        .status(StatusCode::NOT_FOUND)
        .body(Empty::new().boxed())
}

/// error response on http INTERNAL_SERVER_ERROR (500)
pub fn internal_server_error_response(
    msg: &str,
) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
    default_builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Full::new(Bytes::from(msg.to_owned())).boxed())
}
