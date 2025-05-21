/// app entry point on executable
#[tokio::main]
async fn main() {
    let app = apimock::run(apimock::core::args::EnvArgs::init_with_default()).await;
    app.server.start().await
}
