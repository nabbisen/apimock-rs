use http_body_util::{BodyExt, Full};
use hyper::{body::Bytes, StatusCode};

use crate::core::server::{response::default_builder, types::BoxBody};

use super::Respond;

/// rule set text response
pub fn text_response(respond: &Respond) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
    let mut builder = default_builder();

    if let Some(headers) = respond.headers.as_ref() {
        builder = headers
            .iter()
            .fold(builder, |builder, (header_name, header_value)| {
                builder.header(header_name, header_value.clone().unwrap_or_default())
            });
    }

    let body = respond.content.as_str();

    builder
        .status(StatusCode::OK)
        .body(Full::new(Bytes::from(body.to_owned())).boxed())
}
