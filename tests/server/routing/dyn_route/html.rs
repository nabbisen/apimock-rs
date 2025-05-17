use hyper::StatusCode;

use crate::util::{http_response, response_body_str, setup};

#[tokio::test]
async fn dyn_data_dir_html() {
    let port = setup().await;
    let response = http_response("/html/index.html", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");

    let body_str = response_body_str(response).await;
    assert_eq!(
        body_str.as_str(),
        "<!DOCTYPE html>\nHello from API mock (apimock-rs)"
    );
}
