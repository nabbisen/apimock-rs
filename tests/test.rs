use apimock::{core::app::App, core::constant::config::DEFAULT_LISTEN_PORT};

use http_body_util::{BodyExt, Empty, Full};
use hyper::{
    body::{Bytes, Incoming},
    Request, Response, StatusCode, Uri,
};
use hyper_util::rt::TokioIo;
use std::path::Path;
use tokio::net::TcpStream;

#[tokio::test]
async fn uri_root_as_empty() {
    setup("apimock.toml").await;
    let response = http_response("", None).await;

    assert_eq!(response.status(), StatusCode::OK);
    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"hello\":\"world\"}");
}

#[tokio::test]
async fn uri_root() {
    setup("apimock.toml").await;
    let response = http_response("/", None).await;

    assert_eq!(response.status(), StatusCode::OK);
    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"hello\":\"world\"}");
}

#[tokio::test]
async fn api_root_as_empty() {
    setup("apimock.toml").await;
    let response = http_response("/api/v1", None).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn api_root() {
    setup("apimock.toml").await;
    let response = http_response("/api/v1/", None).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn api_home() {
    setup("apimock.toml").await;
    let response = http_response("/api/v1/home", None).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"key\":\"value\"}");
}

#[tokio::test]
async fn matcher_object_1() {
    setup("apimock.toml").await;
    let body = "{\"a\":{\"b\":{\"c\":\"1\"}}}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body)).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"apikey\":\"apivalue\"}");
}

#[tokio::test]
async fn matcher_object_2() {
    setup("apimock.toml").await;
    let body = "{\"a\":{\"b\":{\"c\":\"0\"}}}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body)).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"key\":\"value\"}");
}

#[tokio::test]
async fn matcher_object_3() {
    setup("apimock.toml").await;
    let body = "{\"a\":{\"b\":{\"c\":\"1\", \"d\": 0}}}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body)).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"apikey\":\"apivalue\"}");
}

#[tokio::test]
async fn matcher_data_type_insensitiveness() {
    setup("apimock.toml").await;
    let body = "{\"a\":{\"b\":{\"c\":1}}}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body)).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"apikey\":\"apivalue\"}");
}

#[tokio::test]
async fn matcher_object_missing() {
    setup("apimock.toml").await;
    let body = "{\"a\":{\"b\":{\"c\":\"2\"}}}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body)).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"key\":\"value\"}");
}

#[tokio::test]
async fn matcher_array() {
    setup("apimock.toml").await;
    let body = "{\"d\":[{},{},{\"e\":\"x=\"}]}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body)).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"apikey\":\"apivalue\"}");
}

#[tokio::test]
async fn matcher_array_missing() {
    setup("apimock.toml").await;
    let body = "{\"d\":[{\"e\":\"x=\"}]}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body)).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"key\":\"value\"}");
}

#[tokio::test]
async fn matcher_empty_value() {
    setup("apimock.toml").await;
    let body = "{\"f\":\"\"}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body)).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"apikey\":\"apivalue\"}");
}

#[tokio::test]
async fn error401() {
    setup("apimock.toml").await;
    let response = http_response("/api/v1/error/401", None).await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn error403() {
    setup("apimock.toml").await;
    let response = http_response("/api/v1/error/api-403", None).await;

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

// utils
/// test initial setup: start up mock server
async fn setup(config_file: &str) {
    if !Path::new(config_file).exists() {
        panic!("config file was missing: {}", config_file);
    }
    let config_file = config_file.to_owned();
    tokio::spawn(async move {
        let server = App::new(config_file.as_str(), None, true).await;
        server.start().await
    });
    // wait for server started
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}

/// get http response from mock server
async fn http_response(uri_path: &str, body: Option<&str>) -> Response<Incoming> {
    let uri: Uri = Uri::builder()
        .scheme("http")
        .authority("127.0.0.1".to_string() + ":" + &DEFAULT_LISTEN_PORT.to_string())
        .path_and_query(uri_path)
        .build()
        .unwrap();

    let host = uri.host().expect("uri has no host");
    let port = uri.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect(addr).await.unwrap();
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
