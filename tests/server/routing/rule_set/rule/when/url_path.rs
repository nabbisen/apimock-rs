use hyper::StatusCode;

use crate::util::{http_response, response_body_str, setup};

#[tokio::test]
async fn url_root() {
    let port = setup().await;
    let response = http_response("/", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"hello\":\"world\"}");
}

#[tokio::test]
async fn url_root_as_empty() {
    let port = setup().await;
    let response = http_response("", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"hello\":\"world\"}");
}

#[tokio::test]
async fn api_root() {
    let port = setup().await;
    let response = http_response("/url-path/", None, port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn api_root_as_empty() {
    let port = setup().await;
    let response = http_response("/url-path", None, port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn api_home() {
    let port = setup().await;
    let response = http_response("/url-path/home", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"hello\":\"world\"}");
}
