use hyper::StatusCode;

use crate::{
    constant::DUMMY_BINARY_DATA,
    util::{
        http::{test_request::TestRequest, test_response::response_body_bytes},
        test_setup::TestSetup,
    },
};

#[tokio::test]
async fn dyn_data_dir_video_mp4() {
    let port = TestSetup::default().launch().await;

    let response = TestRequest::default("/binary/video/video.mp4", port)
        .send()
        .await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(response.headers().get("content-type").unwrap(), "video/mp4");

    let body_str = response_body_bytes(response).await;
    assert_eq!(body_str.as_ref(), DUMMY_BINARY_DATA);
}
