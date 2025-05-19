//! API mock Server generating HTTP/JSON responses
//!
//! Mocking helper to develop microservices and APIs.
//! [hyper](https://hyper.rs/)-based HTTP server generating REST responses containing JSON ones.

pub mod core;
use core::app::App;
use core::args::EnvArgs;

#[cfg(feature = "napi")]
use napi_derive::napi;

/// return hyper http server
#[cfg(not(feature = "spawn"))]
pub async fn run(env_args: EnvArgs) -> App {
    App::new(env_args, None, true).await
}

#[cfg(feature = "spawn")]
use tokio::sync::mpsc::Sender;

/// accept sender to main proc set to logger and
/// return hyper http server
/// `includes_ansi_codes`: if true, log includes ansi escape codes for console text color
#[cfg(feature = "spawn")]
pub async fn run(env_args: EnvArgs, spawn_tx: Sender<String>, includes_ansi_codes: bool) -> App {
    App::new(env_args, Some(spawn_tx), includes_ansi_codes).await
}

/// node.js binding entry point
#[cfg(feature = "napi")]
#[napi]
pub async fn napi_run() {
    let _ = run(EnvArgs::init_with_default()).await;
}
