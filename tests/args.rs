use hyper::StatusCode;
use serde_json::json;
use util::{
    http::{test_request::TestRequest, test_response::response_body_str},
    test_setup::TestSetup,
};

#[path = "util.rs"]
mod util;

#[tokio::test]
async fn port_env_arg_overwrites() {
    let port = u16::MAX;
    let mut test_setup = TestSetup::default();
    test_setup.port = Some(port);
    let _ = test_setup.launch().await;

    let response = TestRequest::default("/", port).send().await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), json!({"hello": "index"}).to_string());
}
