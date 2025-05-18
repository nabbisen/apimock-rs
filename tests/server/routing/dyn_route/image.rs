use hyper::StatusCode;

use crate::util::{http_response_default, response_body_bytes, setup};

#[tokio::test]
async fn dyn_data_dir_image() {
    let port = setup().await;
    let response = http_response_default("/img/image.png", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/octet-stream"
    );

    let body_str = response_body_bytes(response).await;
    assert_eq!(
        body_str.as_ref(),
        b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR\0\0\0 \0\0\0 \x01\x03\0\0\0I\xb4\xe8\xb7\0\0\0\x03PLTE\xea\xf22\xedR\xba\x13\0\0\0\x0cIDAT\x08\xd7c`\x18\xdc\0\0\0\xa0\0\x01a%}G\0\0\0\0IEND\xaeB`\x82"
    );
}
