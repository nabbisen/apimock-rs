//! API mock Server generating HTTP/JSON responses
//!
//! Mocking helper to develop microservices and APIs.
//! [hyper](https://hyper.rs/)-based HTTP server generating REST responses containing JSON ones.

pub mod core;
use core::apimock::ApiMock;

/// start hyper http server
pub async fn start_server(
    config_path: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ApiMock::start_server(config_path).await
}
