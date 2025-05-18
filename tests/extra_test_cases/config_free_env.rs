use apimock::core::server::constant::CSV_RECORDS_DEFAULT_KEY;
use hyper::StatusCode;
use serde_json::json;

use crate::util::{http_response_default, response_body_str, setup_as_config_free_env};

#[tokio::test]
async fn config_free_env_root_1() {
    let port = setup_as_config_free_env().await;
    let response = http_response_default("/", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn matches_config_free_env_level1_1() {
    let port = setup_as_config_free_env().await;
    let response = http_response_default("/level1", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"level":"1"}).to_string());
}

#[tokio::test]
async fn matches_config_free_env_level1_2() {
    let port = setup_as_config_free_env().await;
    let response = http_response_default("/level1.json", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"level":"1"}).to_string());
}

#[tokio::test]
async fn not_matches_config_free_env_level1_1() {
    let port = setup_as_config_free_env().await;
    let response = http_response_default("/level1.json5", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn matches_config_free_env_level2_1() {
    let port = setup_as_config_free_env().await;
    let response = http_response_default("/level1/level2", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"level":"2"}).to_string());
}

#[tokio::test]
async fn matches_config_free_env_level2_2() {
    let port = setup_as_config_free_env().await;
    let response = http_response_default("/level1/level2.json5", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"level":"2"}).to_string());
}

#[tokio::test]
async fn not_matches_config_free_env_level2_1() {
    let port = setup_as_config_free_env().await;
    let response = http_response_default("/level1/level2.json", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn matches_config_free_env_level3_1() {
    let port = setup_as_config_free_env().await;
    let response = http_response_default("/level1/level2/level3", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(
        body_str.as_str(),
        json!({CSV_RECORDS_DEFAULT_KEY:[{"level":"3"}]}).to_string()
    );
}

#[tokio::test]
async fn matches_config_free_env_level3_2() {
    let port = setup_as_config_free_env().await;
    let response = http_response_default("/level1/level2/level3.csv", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(
        body_str.as_str(),
        json!({CSV_RECORDS_DEFAULT_KEY:[{"level":"3"}]}).to_string()
    );
}

#[tokio::test]
async fn not_matches_config_free_env_level3_1() {
    let port = setup_as_config_free_env().await;
    let response = http_response_default("/level1/level2/level3.json", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn matches_config_free_env_level4_1() {
    let port = setup_as_config_free_env().await;
    let response = http_response_default("/level1/level2/level3/level4.txt", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "text/plain"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "level 4");
}

#[tokio::test]
async fn not_matches_config_free_env_level4_1() {
    let port = setup_as_config_free_env().await;
    let response = http_response_default("/level1/level2/level3/level4", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn not_matches_config_free_env_level4_2() {
    let port = setup_as_config_free_env().await;
    let response = http_response_default("/level1/level2/level3/level4.json", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
