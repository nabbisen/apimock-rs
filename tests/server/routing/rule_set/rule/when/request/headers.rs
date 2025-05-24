use std::str::FromStr;

use hyper::{
    http::header::{HeaderName, HeaderValue},
    HeaderMap, StatusCode,
};
use serde_json::json;

use crate::util::{http_response_headers_condition, response_body_str, setup};

#[tokio::test]
async fn match_headers_key_1() {
    let port = setup().await;
    let headers: HeaderMap<HeaderValue> = [("user", "user1")]
        .iter()
        .map(|(k, v)| {
            (
                HeaderName::from_str(k).expect("failed to define header name"),
                HeaderValue::from_static(v),
            )
        })
        .collect();
    let response = http_response_headers_condition("/headers", port, &headers).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"hello": "world"}).to_string());
}

#[tokio::test]
async fn match_headers_key_2() {
    let port = setup().await;
    let headers: HeaderMap<HeaderValue> = [("User", "user1")]
        .iter()
        .map(|(k, v)| {
            (
                HeaderName::from_str(k).expect("failed to define header name"),
                HeaderValue::from_static(v),
            )
        })
        .collect();
    let response = http_response_headers_condition("/headers", port, &headers).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"hello": "world"}).to_string());
}

#[tokio::test]
async fn match_headers_key_3() {
    let port = setup().await;
    let headers: HeaderMap<HeaderValue> = [("uSER", "user1")]
        .iter()
        .map(|(k, v)| {
            (
                HeaderName::from_str(k).expect("failed to define header name"),
                HeaderValue::from_static(v),
            )
        })
        .collect();
    let response = http_response_headers_condition("/headers", port, &headers).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"hello": "world"}).to_string());
}

#[tokio::test]
async fn match_headers_key_4() {
    let port = setup().await;
    let headers: HeaderMap<HeaderValue> = [("USER", "user1")]
        .iter()
        .map(|(k, v)| {
            (
                HeaderName::from_str(k).expect("failed to define header name"),
                HeaderValue::from_static(v),
            )
        })
        .collect();
    let response = http_response_headers_condition("/headers", port, &headers).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"hello": "world"}).to_string());
}

#[tokio::test]
async fn not_match_headers_key_2() {
    let port = setup().await;
    let headers: HeaderMap<HeaderValue> = [("user", "user2")]
        .iter()
        .map(|(k, v)| (HeaderName::from_static(k), HeaderValue::from_static(v)))
        .collect();
    let response = http_response_headers_condition("/headers", port, &headers).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
