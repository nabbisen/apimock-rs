#![allow(dead_code)]

// inner attribute above suppresses unused warns for pub fns
// as test mods are compiled separately in rust and therefore the compiler doesn’t consider external calls

use http_body_util::{BodyExt, Empty, Full};
use hyper::{
    body::{Bytes, Incoming},
    header::{HeaderMap, HeaderValue},
    Request, Response, Uri,
};
use hyper_util::rt::TokioIo;
use rand::Rng;
use tokio::net::TcpStream;

use std::env;

use apimock::core::{app::App, args::EnvArgs};

const TEST_WORKDIR: &str = "examples/config/tests";
const CONFIG_FILE_PATH: &str = "apimock.toml";
pub const DYN_ROUTE_DIR: &str = "apimock-dyn-route";
const MIDDLEWARE_FILE_PATH: &str = "apimock-middleware.rhai";

/// test initial setup: start up mock server
pub async fn setup() -> u16 {
    let port = dynamic_port();
    setup_with_port(port).await;
    port
}

/// test initial setup: start up mock server with specific port number
pub async fn setup_with_port(port: u16) {
    let _ = env::set_current_dir(TEST_WORKDIR);

    tokio::spawn(async move {
        let app = App::new(env_args(port), None, true).await;
        app.server.start().await
    });
    // wait for server started
    tokio::time::sleep(std::time::Duration::from_millis(400)).await;
}

/// env args for testing
fn env_args(port: u16) -> EnvArgs {
    let mut ret = EnvArgs::init_with_default();

    ret.config_file_path = Some(CONFIG_FILE_PATH.to_owned());
    ret.port = Some(port);
    ret.middleware_file_path = Some(MIDDLEWARE_FILE_PATH.to_owned());

    match ret.validate() {
        Ok(_) => ret,
        Err(_) => panic!("something wrong in env args"),
    }
}

/// select dynamic port randomly
fn dynamic_port() -> u16 {
    rand::rng().random_range(49152..=65535)
}

/// get http response from mock server
pub async fn http_response_default(url_path: &str, port: u16) -> Response<Incoming> {
    http_response(url_path, port, None, None).await
}

/// get http response from mock server
pub async fn http_response_headers_condition(
    url_path: &str,
    port: u16,
    headers: &HeaderMap<HeaderValue>,
) -> Response<Incoming> {
    http_response(url_path, port, Some(headers), None).await
}

/// get http response from mock server
pub async fn http_response_body_condition(
    url_path: &str,
    port: u16,
    body: &str,
) -> Response<Incoming> {
    http_response(url_path, port, None, Some(body)).await
}

/// get http response from mock server
async fn http_response(
    url_path: &str,
    port: u16,
    headers: Option<&HeaderMap<HeaderValue>>,
    body: Option<&str>,
) -> Response<Incoming> {
    let url: Uri = Uri::builder()
        .scheme("http")
        .authority(format!("127.0.0.1:{}", port.to_string()))
        .path_and_query(url_path)
        .build()
        .unwrap();

    let host = url.host().expect("url has no host");
    let port = url.port_u16().expect("some problem around port");
    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect(addr)
        .await
        .expect(format!("tcp connect failed with {}:{}", host, port).as_str());
    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await.unwrap();
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            log::error!("Connection failed: {:?}", err);
        }
    });

    let authority = url.authority().unwrap().clone();

    let path = url.path();
    let body = if body.is_none() {
        Empty::new().boxed()
    } else {
        Full::new(Bytes::from(body.unwrap().to_owned())).boxed()
    };
    let mut builder = Request::builder()
        .uri(path)
        .header(hyper::header::HOST, authority.as_str());
    if let Some(headers) = headers {
        for (header_key, header_value) in headers.iter() {
            builder = builder.header(header_key, header_value);
        }
    }
    let req = builder.body(body).expect("failed to create http request");

    sender.send_request(req).await.unwrap()
}

/// convert response body bytes to string
pub async fn response_body_str(response: Response<Incoming>) -> String {
    let body_bytes = response_body_bytes(response).await;
    let body_str = String::from_utf8(body_bytes.into()).unwrap();
    body_str
}

/// convert response body bytes to string
pub async fn response_body_bytes(response: Response<Incoming>) -> Bytes {
    response
        .into_body()
        .boxed()
        .collect()
        .await
        .unwrap()
        .to_bytes()
}
