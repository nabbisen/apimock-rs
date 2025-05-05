use apimock::core::app::App;

use http_body_util::{BodyExt, Empty, Full};
use hyper::{
    body::{Bytes, Incoming},
    Request, Response, StatusCode, Uri,
};
use hyper_util::rt::TokioIo;
use rand::Rng;
use std::{env, path::Path};
use tokio::net::TcpStream;

// todo: config -> default or something ?
const TEST_WORKDIR: &str = "examples/config";
const CONFIG_FILEPATH: &str = "apimock.toml";
const MIDDLEWARE_FILEPATH: &str = "middleware.rhai";

#[tokio::test]
async fn uri_root_as_empty() {
    let port = setup().await;
    let response = http_response("", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);
    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"hello\":\"world\"}");
}

#[tokio::test]
async fn uri_root() {
    let port = setup().await;
    let response = http_response("/", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);
    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"hello\":\"world\"}");
}

#[tokio::test]
async fn api_root_as_empty() {
    let port = setup().await;
    let response = http_response("/api/v1", None, port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn api_root() {
    let port = setup().await;
    let response = http_response("/api/v1/", None, port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn api_home() {
    let port = setup().await;
    let response = http_response("/api/v1/home", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"key\":\"value\"}");
}

#[tokio::test]
async fn matcher_object_1() {
    let port = setup().await;
    let body = "{\"a\":{\"b\":{\"c\":\"1\"}}}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"apikey\":\"apivalue\"}");
}

#[tokio::test]
async fn matcher_object_2() {
    let port = setup().await;
    let body = "{\"a\":{\"b\":{\"c\":\"0\"}}}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"key\":\"value\"}");
}

#[tokio::test]
async fn matcher_object_3() {
    let port = setup().await;
    let body = "{\"a\":{\"b\":{\"c\":\"1\", \"d\": 0}}}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"apikey\":\"apivalue\"}");
}

#[tokio::test]
async fn matcher_data_type_insensitiveness() {
    let port = setup().await;
    let body = "{\"a\":{\"b\":{\"c\":1}}}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"apikey\":\"apivalue\"}");
}

#[tokio::test]
async fn matcher_object_missing() {
    let port = setup().await;
    let body = "{\"a\":{\"b\":{\"c\":\"2\"}}}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"key\":\"value\"}");
}

#[tokio::test]
async fn matcher_array() {
    let port = setup().await;
    let body = "{\"d\":[{},{},{\"e\":\"x=\"}]}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"apikey\":\"apivalue\"}");
}

#[tokio::test]
async fn matcher_array_missing() {
    let port = setup().await;
    let body = "{\"d\":[{\"e\":\"x=\"}]}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"key\":\"value\"}");
}

#[tokio::test]
async fn matcher_empty_value() {
    let port = setup().await;
    let body = "{\"f\":\"\"}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"apikey\":\"apivalue\"}");
}

#[tokio::test]
async fn error401() {
    let port = setup().await;
    let response = http_response("/api/v1/error/401", None, port).await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn error403() {
    let port = setup().await;
    let response = http_response("/api/v1/error/api-403", None, port).await;

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

// utils
/// test initial setup: start up mock server
async fn setup() -> u16 {
    let port = dynamic_port();

    let _ = env::set_current_dir(TEST_WORKDIR);

    let config_filepath = CONFIG_FILEPATH;
    // todo: preapre .rhai if necessary
    let middleware_filepath = Some(MIDDLEWARE_FILEPATH.to_owned());

    if !Path::new(config_filepath).exists() {
        panic!("config file was missing: {}", config_filepath);
    }

    tokio::spawn(async move {
        let server = App::new(config_filepath, Some(port), middleware_filepath, None, true).await;
        server.start().await
    });
    // wait for server started
    tokio::time::sleep(std::time::Duration::from_millis(400)).await;

    port
}

/// select dynamic port randomly
fn dynamic_port() -> u16 {
    rand::rng().random_range(49152..=65535)
}

/// get http response from mock server
async fn http_response(uri_path: &str, body: Option<&str>, port: u16) -> Response<Incoming> {
    let uri: Uri = Uri::builder()
        .scheme("http")
        .authority(format!("127.0.0.1:{}", port.to_string()))
        .path_and_query(uri_path)
        .build()
        .unwrap();

    let host = uri.host().expect("uri has no host");
    let port = uri.port_u16().expect("some problem around port");
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

    let authority = uri.authority().unwrap().clone();

    let path = uri.path();
    let body = if body.is_none() {
        Empty::new().boxed()
    } else {
        Full::new(Bytes::from(body.unwrap().to_owned())).boxed()
    };
    let req = Request::builder()
        .uri(path)
        .header(hyper::header::HOST, authority.as_str())
        .body(body)
        .unwrap();

    sender.send_request(req).await.unwrap()
}

/// convert response body bytes to string
async fn response_body_str(response: Response<Incoming>) -> String {
    let body_bytes = response
        .into_body()
        .boxed()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let body_str = String::from_utf8(body_bytes.into()).unwrap();
    body_str
}
