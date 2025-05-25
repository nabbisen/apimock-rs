use console::style;
use hyper::{body, service::service_fn};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
};
use response::error_response::internal_server_error_response;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::Arc;

pub mod constant;
pub mod middleware;
pub mod parsed_request;
pub mod response;
pub mod routing;
mod routing_analysis;
pub mod types;

use crate::core::app::app_state::AppState;
use crate::core::app::constant::APP_NAME;
use parsed_request::ParsedRequest;
use routing::dyn_route::dyn_route_content;
use types::BoxBody;

/// server
pub struct Server {
    pub addr: SocketAddr,
    pub listener: TcpListener,
    pub app_state: AppState,
}

impl Server {
    pub async fn new(app_state: AppState) -> Self {
        let addr = app_state
            .config
            .listener_address()
            .to_socket_addrs()
            .expect("invalid listend address or port")
            .next()
            .expect("failed to resolve address");
        let listener = TcpListener::bind(addr)
            .await
            .expect("tcp listener failed to bind address");

        Server {
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
                        service_fn(move |request: hyper::Request<body::Incoming>| {
                            service(request, app_state.clone())
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

/// entry point of http requests handler service
pub async fn service(
    request: hyper::Request<body::Incoming>,
    app_state: Arc<Mutex<AppState>>,
) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
    let request = match ParsedRequest::from(request).await {
        Ok(x) => x,
        Err(err) => return internal_server_error_response(err.as_str()),
    };

    let shared_app_state = { app_state.lock().await.clone() };

    // app handle driven by config
    let config = shared_app_state.config;

    request.capture_in_log(config.log.unwrap_or_default().verbose);

    match config.service.middleware_response(&request) {
        Some(x) => return x,
        None => (),
    }

    match config.service.rule_set_response(&request).await {
        Some(x) => return x,
        None => (),
    }

    dyn_route_content(
        request.url_path.as_str(),
        config.service.fallback_respond_dir.as_str(),
    )
}
