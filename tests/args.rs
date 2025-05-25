use hyper::StatusCode;
use util::{
    http::{http_response_default, response_body_str},
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

    let response = http_response_default("/", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"hello\":\"world\"}");
}
