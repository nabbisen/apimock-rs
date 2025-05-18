use hyper::StatusCode;

#[path = "util.rs"]
mod util;

use util::{http_response_default, response_body_str, setup_with_port};

#[tokio::test]
async fn port_env_arg_overwrites() {
    let port = u16::MAX;
    setup_with_port(port).await;

    let response = http_response_default("/", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"hello\":\"world\"}");
}
