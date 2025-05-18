use hyper::StatusCode;

use crate::util::{http_response_default, setup};

#[tokio::test]
async fn json_w_ext() {
    let port = setup().await;
    let response = http_response_default("/file-response", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    // assert_eq!(
    //     response.headers().get("content-type").unwrap(),
    //     "application/json"
    // );

    // let body_str = response_body_str(response).await;
    // assert_eq!(body_str.as_str(), "{\"hello\":\"world\"}");
}

// #[tokio::test]
// async fn json_wo_ext() {
//     let port = setup().await;
//     let response = http_response_default("/file-response", port).await;

//     assert_eq!(response.status(), StatusCode::OK);
// }

// #[tokio::test]
// async fn json5_w_ext() {
//     let port = setup().await;
//     let response = http_response_default("/file-response", port).await;

//     assert_eq!(response.status(), StatusCode::OK);
// }

// #[tokio::test]
// async fn json5_wo_ext() {
//     let port = setup().await;
//     let response = http_response_default("/file-response", port).await;

//     assert_eq!(response.status(), StatusCode::OK);
// }

// #[tokio::test]
// async fn csv_w_ext() {
//     let port = setup().await;
//     let response = http_response_default("/file-response", port).await;

//     assert_eq!(response.status(), StatusCode::OK);
// }

// #[tokio::test]
// async fn csv_wo_ext() {
//     let port = setup().await;
//     let response = http_response_default("/file-response", port).await;

//     assert_eq!(response.status(), StatusCode::OK);
// }

// #[tokio::test]
// async fn other_w_ext() {
//     let port = setup().await;
//     let response = http_response_default("/file-response", port).await;

//     assert_eq!(response.status(), StatusCode::OK);
// }

// #[tokio::test]
// async fn other_wo_ext() {
//     let port = setup().await;
//     let response = http_response_default("/file-response", port).await;

//     assert_eq!(response.status(), StatusCode::OK);
// }
