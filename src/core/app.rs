use console::style;
use hyper::body::Bytes;
use hyper::{body, service::service_fn, Request, Response};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
};
use rhai::Engine;
use tokio::sync::mpsc::Sender;
use tokio::{net::TcpListener, sync::Mutex};

use std::convert::Infallible;
use std::net::{SocketAddr, ToSocketAddrs};
use std::path::Path;
use std::sync::Arc;

pub mod app_state;
pub mod constant;

use super::args::EnvArgs;
use super::config::Config;
use super::logger::init_logger;
use super::server::middleware::Middleware;
use super::server::Server;
use app_state::AppState;

/// app
pub struct App {
    pub server: Server,
}

impl App {
    /// create new app
    ///
    /// - listener_port_to_overwrite: ignores port in config toml. used in both arguments and tests
    pub async fn new(
        env_args: EnvArgs,
        spawn_tx: Option<Sender<String>>,
        includes_ansi_codes: bool,
    ) -> Self {
        let _ = init_logger(spawn_tx, includes_ansi_codes);

        let mut config = Config::new(env_args.config_filepath.as_ref());

        // overwrite port if the arg is specified
        if let Some(port) = env_args.port {
            config.port = port;
        }

        let middleware = Middleware::new(env_args.middleware_filepath.as_ref());

        let app_state = AppState { config, middleware };

        let server = Server::new(app_state).await;

        Self { server }
    }
}
