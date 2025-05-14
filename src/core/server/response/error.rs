use http_body_util::{BodyExt, Empty, Full};
use hyper::{body::Bytes, http::Error, StatusCode};

use super::default;
use crate::core::server::types::BoxBody;

/// error response on http BAD_REQUEST (400)
pub fn bad_request(msg: &str) -> Result<hyper::Response<BoxBody>, Error> {
    default()
        .status(StatusCode::BAD_REQUEST)
        .body(Full::new(Bytes::from(msg.to_owned())).boxed())
}

/// error response on http NOT_FOUND (404)
pub fn not_found() -> Result<hyper::Response<BoxBody>, Error> {
    default()
        .status(StatusCode::NOT_FOUND)
        .body(Empty::new().boxed())
}

/// error response on http INTERNAL_SERVER_ERROR (500)
pub fn internal_server_error(msg: &str) -> Result<hyper::Response<BoxBody>, Error> {
    default()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Full::new(Bytes::from(msg.to_owned())).boxed())
}
