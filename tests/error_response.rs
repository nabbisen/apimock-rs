use hyper::StatusCode;

#[cfg(test)]
mod util;

use util::{http_response, setup};

#[tokio::test]
async fn error401() {
    let port = setup().await;
    let response = http_response("/api/v1/error/401", None, port).await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn error403() {
    let port = setup().await;
    let response = http_response("/api/v1/error/api-403", None, port).await;

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}
