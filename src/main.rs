use apimock::{
    core::{
        app_state::middleware_filepath,
        config::{config_filepath, config_listener_port},
    },
    server,
};

/// app entry point on executable
#[tokio::main]
async fn main() {
    let server = server(
        config_filepath().as_str(),
        config_listener_port(),
        Some(middleware_filepath().to_owned()),
    )
    .await;
    server.start().await
}
