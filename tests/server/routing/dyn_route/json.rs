use hyper::StatusCode;

use crate::util::{http_response_default, response_body_str, setup};

#[tokio::test]
async fn dyn_data_dir_json_root_json_ext_none() {
    let port = setup().await;
    let response = http_response_default("/root1", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"name\":\"root1.json\"}");
}

#[tokio::test]
async fn dyn_data_dir_json_root_json_ext_json() {
    let port = setup().await;
    let response = http_response_default("/root1.json", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"name\":\"root1.json\"}");
}

#[tokio::test]
async fn dyn_data_dir_json_root_json_ext_json5() {
    let port = setup().await;
    let response = http_response_default("/root1.json5", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"name\":\"root1.json5\"}");
}

#[tokio::test]
async fn dyn_data_dir_json_root_json5() {
    let port = setup().await;
    let response = http_response_default("/root1.json5", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"name\":\"root1.json5\"}");
}

#[tokio::test]
async fn dyn_data_dir_json_root_multiple() {
    let port = setup().await;
    let response = http_response_default("/root2.json", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"name\":\"root2.json\"}");
}

#[tokio::test]
async fn dyn_data_dir_json_subdir() {
    let port = setup().await;
    let response = http_response_default("/json/subdir.json", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"name\":\"subdir.json\"}");
}

#[tokio::test]
async fn dyn_data_dir_json_depth() {
    let port = setup().await;
    let response = http_response_default("/json/another-dir/depth.json", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"name\":\"depth.json\"}");
}
