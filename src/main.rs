use apimock::{core::args::EnvArgs, server};

/// app entry point on executable
#[tokio::main]
async fn main() {
    let server = server(EnvArgs::init_with_default()).await;
    server.start().await
}
