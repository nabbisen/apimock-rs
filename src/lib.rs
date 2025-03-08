//! API mock Server generating HTTP/JSON responses
//!
//! Mocking helper to develop microservices and APIs.
//! [hyper](https://hyper.rs/)-based HTTP server generating REST responses containing JSON ones.

pub mod core;
use core::app::App;

/// return hyper http server
#[cfg(not(feature = "spawn"))]
pub async fn server(config_path: &str) -> App {
    App::new(config_path, None, true).await
}

#[cfg(feature = "spawn")]
use tokio::sync::mpsc::Sender;

/// accept sender to main proc set to logger and
/// return hyper http server
/// `includes_ansi_codes`: if true, log includes ansi escape codes for console text color
#[cfg(feature = "spawn")]
pub async fn server(config_path: &str, spawn_tx: Sender<String>, includes_ansi_codes: bool) -> App {
    App::new(config_path, Some(spawn_tx), includes_ansi_codes).await
}

/// start hyper http server (deprecated)
#[deprecated]
pub async fn start_server(
    config_path: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    #[allow(deprecated)]
    App::start_server(config_path).await
}
