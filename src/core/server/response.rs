use hyper::{
    header::{
        HeaderValue, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
        ACCESS_CONTROL_ALLOW_ORIGIN, CACHE_CONTROL, CONTENT_TYPE,
    },
    http::response::Builder,
};

pub mod error_response;
pub mod file_response;
pub mod status_code_response;
pub mod text_response;
mod util;

pub fn default_builder() -> Builder {
    let builder = hyper::Response::builder()
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"))
        .header(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"))
        .header(
            ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("GET, POST, OPTIONS, HEAD"),
        )
        .header(CACHE_CONTROL, HeaderValue::from_static("no-store"));
    builder
}

/// response base on json response
pub fn json_builder() -> Builder {
    default_builder().header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
}
