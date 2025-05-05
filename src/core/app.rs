use std::net::{SocketAddr, ToSocketAddrs};
use std::path::Path;
use std::sync::Arc;

use console::style;
use hyper::body::Bytes;
use hyper::{body, service::service_fn, Request, Response};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
};
use rhai::Engine;
use std::convert::Infallible;
use tokio::sync::mpsc::Sender;
use tokio::{net::TcpListener, sync::Mutex};

use super::app_state::{AppState, Middleware};
use super::config::Config;
use super::constant::APP_NAME;
use super::logger::init_logger;
use super::server::handle;

type BoxBody = http_body_util::combinators::BoxBody<Bytes, Infallible>;

/// app
pub struct App {
    addr: SocketAddr,
    listener: TcpListener,
    app_state: AppState,
}

impl App {
    /// create new app
    ///
    /// - listener_port_to_overwrite: ignores port in config toml. used in tests
    pub async fn new(
        config_filepath: &str,
        listener_port_to_overwrite: Option<u16>,
        middleware_filepath: Option<String>,
        spawn_tx: Option<Sender<String>>,
        includes_ansi_codes: bool,
    ) -> Self {
        let _ = init_logger(spawn_tx, includes_ansi_codes);

        let mut config = Config::new(&config_filepath);

        if let Some(port) = listener_port_to_overwrite {
            config.port = port;
        }

        let addr = config
            .listen_address()
            .to_socket_addrs()
            .expect("invalid listend address or port")
            .next()
            .expect("failed to resolve address");
        let listener = TcpListener::bind(addr)
            .await
            .expect("tcp listener failed to bind address");

        let middleware = match middleware_filepath {
            Some(middleware_filepath) if Path::new(middleware_filepath.as_str()).exists() => {
                let engine = Engine::new();
                // todo: watch source file change - `notify` crate ?
                let ast = engine
                    .compile_file(middleware_filepath.clone().into())
                    .expect(
                        format!(
                            "failed to compile middleware file to get ast: {}",
                            middleware_filepath
                        )
                        .as_str(),
                    );

                let middleware = Middleware {
                    engine: Arc::new(engine),
                    filepath: middleware_filepath.to_owned(),
                    ast,
                };

                println!("\nMiddleware is activated: {}", middleware_filepath);
                Some(middleware)
            }
            _ => None,
        };
        let app_state = AppState { config, middleware };

        App {
            addr,
            listener,
            app_state,
        }
    }

    /// app start
    pub async fn start(&self) {
        log::info!(
            "\nGreetings from {APP_NAME} !!\nListening on {} ...\n",
            style(format!("http://{}", self.addr)).cyan()
        );
        let app_state = Arc::new(Mutex::new(self.app_state.clone()));
        loop {
            let (stream, _) = self
                .listener
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
                        service_fn(move |req: Request<body::Incoming>| {
                            service(req, app_state.clone())
                        }),
                    )
                    .await
                {
                    log::error!("error serving connection: {:?}", err);
                }
            });
        }
    }
}

/// handle http service
async fn service(
    req: Request<body::Incoming>,
    app_state: Arc<Mutex<AppState>>,
) -> Result<Response<BoxBody>, hyper::http::Error> {
    handle(req, Arc::clone(&app_state)).await
}
