use hyper::StatusCode;

use crate::{
    constant::root_config_dir,
    util::{
        http::{http_response_default, http_response_json_body_condition, response_body_str},
        test_setup::TestSetup,
    },
};

#[tokio::test]
async fn middleware_url_path_handled() {
    let port = setup().await;

    let response = http_response_default("/middleware-test", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"thisIs\":\"missedByConfigToml\"}");
}

#[tokio::test]
async fn middleware_url_path_missed() {
    let port = setup().await;

    let response = http_response_default("/middleware-test/dummy", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "");
}

#[tokio::test]
async fn middleware_body_handled() {
    let port = setup().await;
    let body = "{\"middleware\": \"isHere\"}";

    let response = http_response_json_body_condition("/middleware-test/dummy", port, body).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"thisIs\":\"missedByConfigToml\"}");
}

#[tokio::test]
async fn middleware_body_missed() {
    let port = setup().await;
    let body = "{\"middleware\": \"isHere?\"}";

    let response = http_response_json_body_condition("/middleware-test/dummy", port, body).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "");
}

/// internal setup fn
async fn setup() -> u16 {
    let test_setup = TestSetup::default_with_root_config_dir(root_config_dir::MIDDLEWARE);
    let port: u16 = test_setup.launch().await;
    port
}
