use hyper::StatusCode;

use crate::{
    constant::{CONFIG_TESTS_ROOT_DIR_PATH, DYN_ROUTE_DIR},
    util::{
        http::{http_response_default, response_body_str},
        test_setup::TestSetup,
    },
};

#[tokio::test]
async fn dyn_data_dir_dir() {
    let port = TestSetup::default().launch().await;

    let response = http_response_default("/html", port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body_str = response_body_str(response).await;
    assert_eq!(
        body_str.as_str(),
        format!(
            "{}/{}/html is not a file. must be missing or a directory",
            CONFIG_TESTS_ROOT_DIR_PATH, DYN_ROUTE_DIR
        )
        .as_str()
    );
}
