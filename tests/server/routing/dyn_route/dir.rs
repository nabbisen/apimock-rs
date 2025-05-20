use hyper::StatusCode;

use crate::util::{http_response_default, response_body_str, setup, DYN_ROUTE_DIR, TEST_WORK_DIR};

#[tokio::test]
async fn dyn_data_dir_dir() {
    let port = setup().await;
    let response = http_response_default("/html", port).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body_str = response_body_str(response).await;
    assert_eq!(
        body_str.as_str(),
        format!(
            "{}/{}/html is not a file. must be missing or a directory",
            TEST_WORK_DIR, DYN_ROUTE_DIR
        )
        .as_str()
    );
}
