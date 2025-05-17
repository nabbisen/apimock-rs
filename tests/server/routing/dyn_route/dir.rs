use hyper::StatusCode;

use crate::util::{http_response, response_body_str, setup, DYN_ROUTE_DIR};

#[tokio::test]
async fn dyn_data_dir_dir() {
    let port = setup().await;
    let response = http_response("/html", None, port).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body_str = response_body_str(response).await;
    assert_eq!(
        body_str.as_str(),
        format!("{}/html is directory", DYN_ROUTE_DIR).as_str()
    );
}
