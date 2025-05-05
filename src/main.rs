use apimock::{core::config::config_filepath, server};

// todo: treat as arg ?
pub const MIDDLEWARE_FILEPATH: &str = "middleware.rhai";

/// app entry point on executable
#[tokio::main]
async fn main() {
    let server = server(
        config_filepath().as_str(),
        Some(MIDDLEWARE_FILEPATH.to_owned()),
    )
    .await;
    server.start().await
}
