use hyper::{
    http::header::{HeaderName, HeaderValue},
    HeaderMap, StatusCode,
};

use crate::util::{http_response_headers_condition, response_body_str, setup};

#[tokio::test]
async fn headers_key_match() {
    let port = setup().await;
    let headers: HeaderMap<HeaderValue> = [("user", "user1")]
        .iter()
        .map(|(k, v)| (HeaderName::from_static(k), HeaderValue::from_static(v)))
        .collect();
    let response = http_response_headers_condition("/headers", port, &headers).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"hello\":\"world\"}");
}

#[tokio::test]
async fn headers_key_not_match() {
    let port = setup().await;
    let headers: HeaderMap<HeaderValue> = [("user", "user2")]
        .iter()
        .map(|(k, v)| (HeaderName::from_static(k), HeaderValue::from_static(v)))
        .collect();
    let response = http_response_headers_condition("/headers", port, &headers).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
