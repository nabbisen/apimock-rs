use hyper::StatusCode;
use serde_json::json;

use crate::util::{http_response_default, response_body_str, setup};

#[tokio::test]
async fn matches_json_wo_ext() {
    let port = setup().await;
    let response = http_response_default("/file-response/json-ext-w-or-wo", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"key": "json"}).to_string());
}

// note: caring about ext existence is somewhat hard on static routing instead of dynamic routing
#[tokio::test]
async fn matches_json_w_ext() {
    let port = setup().await;
    let response = http_response_default("/file-response/json-ext-w-or-wo.json", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn json5_wo_ext() {
    let port = setup().await;
    let response = http_response_default("/file-response/json5-ext-w-or-wo", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"key": "json5"}).to_string());
}

#[tokio::test]
async fn json5_w_ext() {
    let port = setup().await;
    let response = http_response_default("/json5-ext-w-or-wo.json5", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn csv_wo_ext() {
    let port = setup().await;
    let response = http_response_default("/file-response/csv-ext-w-or-wo", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(
        body_str.as_str(),
        json!({"records": [{"a":"key","b":"csv"}]}).to_string()
    );
}

#[tokio::test]
async fn csv_w_ext() {
    let port = setup().await;
    let response = http_response_default("/file-response/csv-ext-w-or-wo.csv", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn other_wo_ext() {
    let port = setup().await;
    let response = http_response_default("/file-response/txt-ext-w-or-wo", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "text/plain"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "text");
}

#[tokio::test]
async fn other_w_ext() {
    let port = setup().await;
    let response = http_response_default("/file-response/txt-ext-w-or-wo.txt", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
