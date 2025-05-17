use hyper::StatusCode;

mod util;

use util::{http_response, response_body_str, setup};

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
async fn raw_path() {
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

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"key\":\"value\"}");
}

#[tokio::test]
async fn matcher_object_1() {
    let port = setup().await;
    let body = "{\"a\":{\"b\":{\"c\":\"1\"}}}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"apikey\":\"apivalue\"}");
}

#[tokio::test]
async fn matcher_object_2() {
    let port = setup().await;
    let body = "{\"a\":{\"b\":{\"c\":\"0\"}}}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"key\":\"value\"}");
}

#[tokio::test]
async fn matcher_object_3() {
    let port = setup().await;
    let body = "{\"a\":{\"b\":{\"c\":\"1\", \"d\": 0}}}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"apikey\":\"apivalue\"}");
}

#[tokio::test]
async fn matcher_data_type_insensitiveness() {
    let port = setup().await;
    let body = "{\"a\":{\"b\":{\"c\":1}}}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"apikey\":\"apivalue\"}");
}

#[tokio::test]
async fn matcher_object_missing() {
    let port = setup().await;
    let body = "{\"a\":{\"b\":{\"c\":\"2\"}}}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"key\":\"value\"}");
}

#[tokio::test]
async fn matcher_array() {
    let port = setup().await;
    let body = "{\"d\":[{},{},{\"e\":\"x=\"}]}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"apikey\":\"apivalue\"}");
}

#[tokio::test]
async fn matcher_array_missing() {
    let port = setup().await;
    let body = "{\"d\":[{\"e\":\"x=\"}]}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"key\":\"value\"}");
}

#[tokio::test]
async fn matcher_empty_value() {
    let port = setup().await;
    let body = "{\"f\":\"\"}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"apikey\":\"apivalue\"}");
}
