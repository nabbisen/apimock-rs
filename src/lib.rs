//! API mock Server generating HTTP/JSON responses
//!
//! Mocking helper to develop microservices and APIs.
//! [hyper](https://hyper.rs/)-based HTTP server generating REST responses containing JSON ones.

pub mod core;
use core::app::App;

const CONFIG_FILEPATH_OPTION_NAMES: [&str; 2] = ["-c", "--config"];
const MIDDLEWARE_FILEPATH_OPTION_NAMES: [&str; 1] = ["--middleware"];

/// return hyper http server
#[cfg(not(feature = "spawn"))]
pub async fn server(config_filepath: &str, middleware_filepath: Option<String>) -> App {
    App::new(config_filepath, middleware_filepath, None, true).await
}

#[cfg(feature = "spawn")]
use tokio::sync::mpsc::Sender;

/// accept sender to main proc set to logger and
/// return hyper http server
/// `includes_ansi_codes`: if true, log includes ansi escape codes for console text color
#[cfg(feature = "spawn")]
pub async fn server(
    config_filepath: &str,
    middleware_filepath: Option<String>,
    spawn_tx: Sender<String>,
    includes_ansi_codes: bool,
) -> App {
    App::new(
        config_filepath,
        middleware_filepath,
        Some(spawn_tx),
        includes_ansi_codes,
    )
    .await
}
