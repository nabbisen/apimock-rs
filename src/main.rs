use apimock::{core::config::config_path, server};

/// app entry point on executable
#[tokio::main]
async fn main() {
    let server = server(config_path().as_str()).await;
    server.start().await
}
