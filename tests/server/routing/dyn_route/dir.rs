use hyper::StatusCode;

use crate::util::{http::http_response_default, test_setup::TestSetup};

#[tokio::test]
async fn dyn_data_dir_dir() {
    let port = TestSetup::default().launch().await;

    let response = http_response_default("/html", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
