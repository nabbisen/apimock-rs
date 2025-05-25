use hyper::StatusCode;

use crate::{
    constant::root_config_dir,
    util::{
        http::{http_response_default, response_body_str},
        test_setup::TestSetup,
    },
};

#[tokio::test]
async fn matches_prefix_url_path_prefix_1() {
    let port = setup().await;

    let response = http_response_default("/prefix/equal", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "text/plain"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str, "url path prefix if");
}

#[tokio::test]
async fn matches_prefix_url_path_prefix_2() {
    let port = setup().await;

    let response = http_response_default("/prefix/equal2", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "text/plain"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str, "url path prefix else");
}

/// internal setup fn
async fn setup() -> u16 {
    let test_setup = TestSetup::default_with_root_config_dir(root_config_dir::RULE_SET_PREFIX);
    let port = test_setup.launch().await;
    port
}
