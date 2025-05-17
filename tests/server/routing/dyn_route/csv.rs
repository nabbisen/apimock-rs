use hyper::StatusCode;

use crate::util::{http_response, response_body_str, setup};

#[tokio::test]
async fn dyn_data_dir_csv() {
    let port = setup().await;
    let response = http_response("/csv/records.csv", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"records\":[{\"fieldA\":\"1\",\"fieldB\":\"2\",\"fieldC\":\"3\"},{\"fieldA\":\"a\",\"fieldB\":\"b\",\"fieldC\":\"c\"},{\"fieldA\":\"#\",\"fieldB\":\"\\\\,\",\"fieldC\":\"!!!\"}]}");
}

#[tokio::test]
async fn dyn_data_dir_csv_wo_ext() {
    let port = setup().await;
    let response = http_response("/csv/records", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"records\":[{\"fieldA\":\"1\",\"fieldB\":\"2\",\"fieldC\":\"3\"},{\"fieldA\":\"a\",\"fieldB\":\"b\",\"fieldC\":\"c\"},{\"fieldA\":\"#\",\"fieldB\":\"\\\\,\",\"fieldC\":\"!!!\"}]}");
}
