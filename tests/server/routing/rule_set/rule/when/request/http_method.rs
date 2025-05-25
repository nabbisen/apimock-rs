use hyper::StatusCode;
use serde_json::json;

use crate::util::{http_get_response_default, http_response_default, response_body_str, setup};

#[tokio::test]
async fn match_http_get_method_1() {
    let port = setup().await;
    let response = http_get_response_default("/http-method", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"method": "get"}).to_string());
}

#[tokio::test]
async fn not_match_http_get_method_1() {
    let port = setup().await;
    let response = http_response_default("/http-method", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

/// internal setup fn
async fn setup() -> u16 {
    let mut test_setup = SetupArgs::default();
    test_setup.current_dir_path = MIDDLEWARE_DIR.to_owned();
    let port = test_setup.setup().await;
    port
}
