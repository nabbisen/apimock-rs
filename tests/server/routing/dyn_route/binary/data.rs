use hyper::StatusCode;

use crate::util::{http_response_default, response_body_bytes, setup, DUMMY_BINARY_DATA};

#[tokio::test]
async fn dyn_data_dir_binary_data() {
    let port = setup().await;
    let response = http_response_default("/binary/data/binary.data", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/octet-stream"
    );

    let body_str = response_body_bytes(response).await;
    assert_eq!(body_str.as_ref(), DUMMY_BINARY_DATA);
}

#[tokio::test]
async fn dyn_data_dir_archive_gzip() {
    let port = setup().await;
    let response = http_response_default("/binary/data/archive.gz", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/octet-stream"
    );

    let body_str = response_body_bytes(response).await;
    assert_eq!(body_str.as_ref(), DUMMY_BINARY_DATA);
}

#[tokio::test]
async fn dyn_data_dir_archive_tar_gzip() {
    let port = setup().await;
    let response = http_response_default("/binary/data/archive.tar.gz", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/octet-stream"
    );

    let body_str = response_body_bytes(response).await;
    assert_eq!(body_str.as_ref(), DUMMY_BINARY_DATA);
}
