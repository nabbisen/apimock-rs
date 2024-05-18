use apimock::start_server;

/// main - app entry point
#[tokio::main]
async fn main() {
    start_server().await.unwrap();
}

