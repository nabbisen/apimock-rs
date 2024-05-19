use apimock::{config::DEFAULT_LISTEN_PORT, start_server};

use std::path::Path;
use hyper::{body::to_bytes, Body, Client, Request, Response, StatusCode, Uri};

#[tokio::test]
async fn uri_root_as_empty() {
    setup("apimock.toml").await;
    let response = http_response("").await;
    
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn uri_root() {
    setup("apimock.toml").await;
    let response = http_response("/").await;
    
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn api_root_as_empty() {
    setup("apimock.toml").await;
    let response = http_response("/api/v1").await;
    
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn api_root() {
    setup("apimock.toml").await;
    let response = http_response("/api/v1/").await;
    
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn api_home() {
    setup("apimock.toml").await;
    let response = http_response("/api/v1/home").await;
    
    assert_eq!(response.status(), StatusCode::OK);

    let body_str = String::from_utf8(to_bytes(response.into_body()).await.unwrap().to_vec()).unwrap();
    assert_eq!(body_str.as_str(), "{\"key\":\"value\"}");
}

#[tokio::test]
async fn error401() {
    setup("apimock.toml").await;
    let response = http_response("/api/v1/error/401").await;
    
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn error403() {
    setup("apimock.toml").await;
    let response = http_response("/api/v1/error/api-403").await;
    
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

// utils
/// test initial setup: start up mock server
async fn setup(config_file: &str) {
    if !Path::new(config_file).exists() {
        panic!("config file was missing: {}", config_file);
    }
    let config_file = config_file.to_owned();
    tokio::spawn(start_server(config_file.to_owned()));
    // wait for server started
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}

/// get http response from mock server
async fn http_response(uri_path: &str) -> Response<Body> {
    let request = http_request(uri_path);
    let client = Client::new();
    let response = client.request(request).await.unwrap();
    response
}

/// generate http request
fn http_request(uri_path: &str) -> Request<Body> {
    let uri = Uri::builder()
        .scheme("http")
        .authority("127.0.0.1".to_string() + ":" + &DEFAULT_LISTEN_PORT.to_string())
        .path_and_query(uri_path)
        .build()
        .unwrap();
    let request = Request::builder()
        .uri(uri)
        .method("POST")
        .header("Content-Type", "text/plain")
        .body(Body::from(""))
        .unwrap();
    request
}