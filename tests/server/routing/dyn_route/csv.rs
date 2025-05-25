use hyper::StatusCode;

use crate::util::{
    http::{http_response_default, response_body_str},
    test_setup::TestSetup,
};

#[tokio::test]
async fn matches_dyn_data_dir_csv() {
    let port = TestSetup::default().launch().await;

    let response = http_response_default("/csv/records.csv", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"records\":[{\"fieldA\":\"1\",\"fieldB\":\"2\",\"fieldC\":\"3\"},{\"fieldA\":\"a\",\"fieldB\":\"b\",\"fieldC\":\"c\"},{\"fieldA\":\"#\",\"fieldB\":\"\\\\,\",\"fieldC\":\"!!!\"}]}");
}

#[tokio::test]
async fn matches_dyn_data_dir_csv_wo_ext() {
    let port = TestSetup::default().launch().await;

    let response = http_response_default("/csv/records", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"records\":[{\"fieldA\":\"1\",\"fieldB\":\"2\",\"fieldC\":\"3\"},{\"fieldA\":\"a\",\"fieldB\":\"b\",\"fieldC\":\"c\"},{\"fieldA\":\"#\",\"fieldB\":\"\\\\,\",\"fieldC\":\"!!!\"}]}");
}
