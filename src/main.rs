use apimock::{core::config::config_path, start_server};

/// main - app entry point
#[tokio::main]
async fn main() {
    start_server(config_path()).await.unwrap();
}
