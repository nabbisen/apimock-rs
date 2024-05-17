use console::style;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::convert::Infallible;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

mod config;
mod server;
use crate::config::Config;
use crate::server::handle;

pub const CONFIG_FILENAME: &str = "apimock.toml";

const APP_NAME: &str = "API mock";

/// main - app entry point
#[tokio::main]
async fn main() {
    println!("\nGreetings from {APP_NAME} !!\n");

    let config_path = config_path();
    let config = Config::new(&config_path);

    let app_state = Arc::new(Mutex::new(config.clone()));
    let make_svc = make_service_fn(|_| {
        let app_state = Arc::clone(&app_state);
        async move {
            let service = service_fn(move |req| handle(req, Arc::clone(&app_state)));
            Ok::<_, Infallible>(service)
        }
    });

    let addr = config.addr.unwrap();
    let server = Server::bind(&addr).serve(make_svc);
    println!(
        "\nListening on {} ...\n",
        style(format!("http://{}", &addr)).cyan()
    );

    server.await.unwrap();
}

/// app config path
/// 
/// - if specified with command-line option, use it
/// - else use the default
fn config_path() -> String {
    let args: Vec<String> = env::args().collect();

    let config_option_entry = args
        .iter()
        .position(|arg| arg.as_str().eq("-c") || arg.as_str().eq("--config"));
    let config_path = match config_option_entry {
        Some(config_option_entry) => match args.get(config_option_entry + 1) {
            Some(config_option) => config_option,
            _ => CONFIG_FILENAME,
        },
        _ => CONFIG_FILENAME,
    };
    config_path.to_owned()
}
