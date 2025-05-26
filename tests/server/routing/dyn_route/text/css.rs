use hyper::StatusCode;

use crate::util::{
    env_args::SetupArgs,
    http::{http_response_default, response_body_bytes},
};

#[tokio::test]
async fn dyn_data_dir_css() {
    let port = TestSetup::default().launch().await;

    let response = http_response_default("/text/css/style.css", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "text/css; charset=utf-8"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(
        body_str.as_str(),
        "html body::after {\n    content: \"Hello from API mock (apimock-rs)\";\n}"
    );
}
