use console::style;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::convert::Infallible;
use std::env;

mod config;
mod server;
use crate::config::Config;
use crate::server::handle;

pub const LISTEN_PORT: u16 = 3001;
pub const CONFIG_FILENAME: &str = "apimock.toml";

#[tokio::main]
async fn main() {
    println!("Greetings from JSON Responder !!");

    let config_path = config_path();
    println!("[config] {}\n", config_path);
    let config = Config::new(&config_path);

    let make_svc = make_service_fn(|_| {
        let config = config.clone();
        async move {
            let service = service_fn(move |req| handle(req, config.clone()));
            Ok::<_, Infallible>(service)
        }
    });

    let addr = ([127, 0, 0, 1], config.port).into();
    let server = Server::bind(&addr).serve(make_svc);
    println!(
        "\nListening on {} ...",
        style(format!("http://{}", addr)).cyan()
    );

    server.await.unwrap();
}

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
