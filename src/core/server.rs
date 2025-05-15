use console::style;
use hyper::{body, service::service_fn};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
};
use response::file_response::FileResponse;
use routing::{dyn_route::dyn_route_content, rule_set::rule_sets_content};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::Arc;

mod constant;
pub mod middleware;
mod parsed_request;
mod response;
pub mod routing;
mod routing_analysis;
mod types;
mod util;

use crate::core::app::app_state::AppState;
use crate::core::app::constant::APP_NAME;
use parsed_request::ParsedRequest;
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
    let request = ParsedRequest::from(request).await;

    let shared_app_state = { app_state.lock().await.clone() };

    // app handle driven by config
    let config = shared_app_state.config;

    request.capture_in_log(config.log.verbose);

    // todo: commands
    // // config update
    // {
    //     let mut shared_app_state = { app_state.lock().await.clone() };
    //     let mut config = shared_app_state.config;

    //     if let Some(x) = handle_data_dir_query_path(&config, request_uri_path.as_str()) {
    //         let res = if !x.is_empty() {
    //             let old_data_dir = config.data_dir.clone().unwrap();
    //             let data_dir = x.strip_prefix("/").unwrap();

    //             config.data_dir = Some(data_dir.to_owned());
    //             config.update_paths(data_dir, old_data_dir.as_str());
    //             shared_app_state.config = config.clone();

    //             plain_text_response(data_dir, None)
    //         } else {
    //             plain_text_response(config.data_dir.clone().unwrap().as_str(), None)
    //         };

    //         request::capture_in_log(
    //             request_uri_path.as_str(),
    //             &parts,
    //             None,
    //             config.verbose.clone(),
    //         );
    //         log::info!(" * [url.data_dir] updated.\n");
    //         config.print_paths();
    //         log::info!("");

    //         return res;
    //     }
    // }

    // middleware if activated
    if let Some(middleware) = shared_app_state.middleware {
        let middleware_response_file_path =
            middleware.handle(request.uri_path.as_str(), request.body_json.as_ref());
        if let Some(middleware_response_file_path) = middleware_response_file_path {
            return FileResponse::new(middleware_response_file_path.as_str(), None)
                .file_content_response();
        }
    }

    match rule_sets_content(&request, &config.service.rule_sets).await {
        Some(x) => return x,
        None => (),
    }

    dyn_route_content(
        request.uri_path.as_str(),
        config.service.fallback_response_dir.as_str(),
    )
}
