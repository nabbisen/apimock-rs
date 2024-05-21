use console::style;
use hyper::{body, body::Bytes, service::service_fn, Request, Response};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
};
use std::convert::Infallible;
use std::env;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::Mutex};

pub mod config;
mod server;
mod util;
use crate::config::Config;
use crate::server::handle;

type BoxBody = http_body_util::combinators::BoxBody<Bytes, Infallible>;

const APP_NAME: &str = "API mock";

/// start hyper http server
pub async fn start_server(
    config_path: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("\nGreetings from {APP_NAME} !!\n");

    let config = Config::new(&config_path);

    let addr = config
        .listen_address()
        .to_socket_addrs()
        .expect("invalid listend address or port")
        .next()
        .expect("failed to resolve address");
    println!(
        "\nListening on {} ...\n",
        style(format!("http://{}", &addr)).cyan()
    );
    let app_state = Arc::new(Mutex::new(config.clone()));
    let listener = TcpListener::bind(addr)
        .await
        .expect("tcp listener failed to bind address");
    loop {
        let (stream, _) = listener
            .accept()
            .await
            .expect("tcp listener failed to accept");
        let io = TokioIo::new(stream);

        let app_state = app_state.clone();
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = Builder::new(TokioExecutor::new())
                // `service_fn` converts our function in a `Service`
                .serve_connection(
                    io,
                    service_fn(move |req: Request<body::Incoming>| service(req, app_state.clone())),
                )
                .await
            {
                eprintln!("error serving connection: {:?}", err);
            }
        });
    }
}

/// server handler service
async fn service(
    req: Request<body::Incoming>,
    app_state: Arc<Mutex<Config>>,
) -> Result<Response<BoxBody>, hyper::http::Error> {
    handle(req, Arc::clone(&app_state)).await
}

/// app config path
///
/// - if specified with command-line option, use it
/// - else use the default
pub fn config_path() -> String {
    let args: Vec<String> = env::args().collect();

    let config_option_entry = args
        .iter()
        .position(|arg| arg.as_str().eq("-c") || arg.as_str().eq("--config"));
    let config_path = match config_option_entry {
        Some(config_option_entry) => match args.get(config_option_entry + 1) {
            Some(config_option) => config_option,
            _ => "",
        },
        _ => "",
    };
    config_path.to_owned()
}
