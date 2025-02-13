use std::net::ToSocketAddrs;
use std::sync::Arc;

use console::style;
use hyper::body::Bytes;
use hyper::{body, service::service_fn, Request, Response};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
};
use std::convert::Infallible;
use tokio::{net::TcpListener, sync::Mutex};

use super::config::Config;
use super::constant::APP_NAME;
use super::server::handle;

type BoxBody = http_body_util::combinators::BoxBody<Bytes, Infallible>;
pub struct ApiMock {}

impl ApiMock {
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
                        service_fn(move |req: Request<body::Incoming>| {
                            service(req, app_state.clone())
                        }),
                    )
                    .await
                {
                    eprintln!("error serving connection: {:?}", err);
                }
            });
        }
    }
}

async fn service(
    req: Request<body::Incoming>,
    app_state: Arc<Mutex<Config>>,
) -> Result<Response<BoxBody>, hyper::http::Error> {
    handle(req, Arc::clone(&app_state)).await
}
