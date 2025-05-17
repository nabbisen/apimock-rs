use hyper::StatusCode;

#[cfg(test)]
mod util;

use util::{http_response, response_body_str, setup};

#[tokio::test]
async fn middleware_uri_path_handled() {
    let port = setup().await;
    let response = http_response("/middleware-test", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"thisIs\":\"missedByConfigToml\"}");
}

#[tokio::test]
async fn middleware_uri_path_missed() {
    let port = setup().await;
    let response = http_response("/middleware-test/dummy", None, port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "");
}

#[tokio::test]
async fn middleware_body_handled() {
    let port = setup().await;
    let body = "{\"middleware\": \"isHere\"}";
    let response = http_response("/middleware-test/dummy", Some(body), port).await;

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
    let response = http_response("/middleware-test/dummy", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "");
}
