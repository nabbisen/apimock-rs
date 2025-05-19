use hyper::{
    header::{HeaderName, HeaderValue},
    HeaderMap, StatusCode,
};
use serde_json::json;

use crate::util::{
    http_response_default, http_response_headers_condition, response_body_str, setup,
};

#[tokio::test]
async fn matches_equal_1() {
    let port: u16 = setup().await;
    let response = http_response_default("/rule-op/equal/1", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"op": "equal"}).to_string());
}

#[tokio::test]
async fn not_matches_equal_1() {
    let port: u16 = setup().await;
    let response = http_response_default("/rule-op/equal/2", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn not_matches_equal_2() {
    let port: u16 = setup().await;
    let response = http_response_default("/rule-op/equal/", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn matches_not_equal_1() {
    let port: u16 = setup().await;
    let headers: HeaderMap<HeaderValue> = [("user", "not-equal-unique-request")]
        .iter()
        .map(|(k, v)| (HeaderName::from_static(k), HeaderValue::from_static(v)))
        .collect();
    let response = http_response_headers_condition("/rule-op/not-equal/2", port, &headers).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"op": "not-equal"}).to_string());
}

#[tokio::test]
async fn matches_not_equal_2() {
    let port: u16 = setup().await;
    let headers: HeaderMap<HeaderValue> = [("user", "not-equal-unique-request")]
        .iter()
        .map(|(k, v)| (HeaderName::from_static(k), HeaderValue::from_static(v)))
        .collect();
    let response = http_response_headers_condition("/rule-op/not-equal/", port, &headers).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"op": "not-equal"}).to_string());
}

#[tokio::test]
async fn not_matches_not_equal_1() {
    let port: u16 = setup().await;
    let headers: HeaderMap<HeaderValue> = [("user", "not-equal-unique-request")]
        .iter()
        .map(|(k, v)| (HeaderName::from_static(k), HeaderValue::from_static(v)))
        .collect();
    let response = http_response_headers_condition("/rule-op/not-equal/1", port, &headers).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn matches_starts_with_1() {
    let port: u16 = setup().await;
    let response = http_response_default("/rule-op/starts-with/12", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"op": "starts-with"}).to_string());
}

#[tokio::test]
async fn matches_starts_with_2() {
    let port: u16 = setup().await;
    let response = http_response_default("/rule-op/starts-with/123", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"op": "starts-with"}).to_string());
}

#[tokio::test]
async fn not_matches_starts_with_1() {
    let port: u16 = setup().await;
    let response = http_response_default("/rule-op/starts-with/1", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn not_matches_starts_with_2() {
    let port: u16 = setup().await;
    let response = http_response_default("/rule-op/starts-with/2", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn not_matches_starts_with_3() {
    let port: u16 = setup().await;
    let response = http_response_default("/rule-op/starts-with/312", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn not_matches_starts_with_4() {
    let port: u16 = setup().await;
    let response = http_response_default("/rule-op/starts-with/", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn matches_contains_1() {
    let port: u16 = setup().await;
    let response = http_response_default("/rule-op/contains/1", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"op": "contains"}).to_string());
}

#[tokio::test]
async fn matches_contains_2() {
    let port: u16 = setup().await;
    let response = http_response_default("/rule-op/contains/12", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"op": "contains"}).to_string());
}

#[tokio::test]
async fn matches_contains_3() {
    let port: u16 = setup().await;
    let response = http_response_default("/rule-op/a/contains/1/b/", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"op": "contains"}).to_string());
}

#[tokio::test]
async fn not_matches_contains_1() {
    let port: u16 = setup().await;
    let response = http_response_default("/rule-op/contains/2", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn matches_wild_card_1() {
    let port: u16 = setup().await;
    let response = http_response_default("/rule-op/wild-card/123/a/", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"op": "wild-card"}).to_string());
}

#[tokio::test]
async fn matches_wild_card_2() {
    let port: u16 = setup().await;
    let response = http_response_default("/rule-op/wild-card/ABC/12/", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"op": "wild-card"}).to_string());
}

#[tokio::test]
async fn matches_wild_card_3() {
    let port: u16 = setup().await;
    let response = http_response_default("/rule-op/wild-card/123", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn not_matches_wild_card_2() {
    let port: u16 = setup().await;

    let response = http_response_default("/rule-op/wild-card/123a", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn not_matches_wild_card_3() {
    let port: u16 = setup().await;
    let response = http_response_default("/rule-op/wild-card/12/a", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn not_matches_wild_card_4() {
    let port: u16 = setup().await;
    let response = http_response_default("/rule-op/wild-card/1234/a", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
