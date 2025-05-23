use hyper::StatusCode;

use crate::util::{http_response_default, response_body_bytes, setup, DUMMY_BINARY_DATA};

#[tokio::test]
async fn dyn_data_dir_video_mp4() {
    let port = setup().await;
    let response = http_response_default("/binary/video/video.mp4", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(response.headers().get("content-type").unwrap(), "video/mp4");

    let body_str = response_body_bytes(response).await;
    assert_eq!(body_str.as_ref(), DUMMY_BINARY_DATA);
}
