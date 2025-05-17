use hyper::StatusCode;

use crate::util::{http_response_default, setup};

#[tokio::test]
async fn error401() {
    let port = setup().await;
    let response = http_response_default("/api/v1/error/401", port).await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn error403() {
    let port = setup().await;
    let response = http_response_default("/api/v1/error/api-403", port).await;

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}
