use hyper::StatusCode;

mod util;

use util::core::{http_response, response_body_bytes, response_body_str, setup};

#[tokio::test]
async fn dyn_data_dir_json_root_json_ext_none() {
    let port = setup().await;
    let response = http_response("/root1", None, port).await;

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
    let response = http_response("/root1.json", None, port).await;

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
    let response = http_response("/root1.json5", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"name\":\"root1.json\"}");
}

#[tokio::test]
async fn dyn_data_dir_json_root_json_ext_csv() {
    let port = setup().await;
    let response = http_response("/root1.csv", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"name\":\"root1.json\"}");
}

#[tokio::test]
async fn dyn_data_dir_json_root_json5() {
    let port = setup().await;
    let response = http_response("/root1.json5", None, port).await;

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
    let response = http_response("/root2.json", None, port).await;

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
    let response = http_response("/json/subdir.json", None, port).await;

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
    let response = http_response("/json/another-dir/depth.json", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"name\":\"depth.json\"}");
}

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
async fn dyn_data_dir_html() {
    let port = setup().await;
    let response = http_response("/html/index.html", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");

    let body_str = response_body_str(response).await;
    assert_eq!(
        body_str.as_str(),
        "<!DOCTYPE html>\nHello from API mock (apimock-rs)"
    );
}

#[tokio::test]
async fn dyn_data_dir_css() {
    let port = setup().await;
    let response = http_response("/css/style.css", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(response.headers().get("content-type").unwrap(), "text/css");

    let body_str = response_body_str(response).await;
    assert_eq!(
        body_str.as_str(),
        "html body::after {\n    content: \"Hello from API mock (apimock-rs)\";\n}"
    );
}

#[tokio::test]
async fn dyn_data_dir_js() {
    let port = setup().await;
    let response = http_response("/js/scripts.js", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/javascript"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(
        body_str.as_str(),
        "function hello() {\n    console.log(\"Hello from API mock (apimock-rs)\")\n}"
    );
}

#[tokio::test]
async fn dyn_data_dir_image() {
    let port = setup().await;
    let response = http_response("/img/image.png", None, port).await;

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

#[tokio::test]
async fn dyn_data_dir_txt() {
    let port = setup().await;
    let response = http_response("/txt/plain.txt", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "text/plain"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "Hello from API mock (apimock-rs)");
}

#[tokio::test]
async fn dyn_data_dir_dir() {
    let port = setup().await;
    let response = http_response("/html", None, port).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "apimock-dyn-data/html is directory");
}
