use http_body_util::{BodyExt, Empty, Full};
use hyper::{
    body::{Bytes, Incoming},
    Request, Response, StatusCode, Uri,
};
use hyper_util::rt::TokioIo;
use rand::Rng;
use std::{env, u16};
use tokio::net::TcpStream;
use util::aaa;

mod util;

use apimock::core::{app::App, args::EnvArgs};

// todo: rename dir "config" -> "default" or something ?
const TEST_WORKDIR: &str = "examples/config/tests";
const CONFIG_FILE_PATH: &str = "apimock.toml";
const MIDDLEWARE_FILE_PATH: &str = "apimock-middleware.rhai";

#[tokio::test]
async fn uri_root_as_empty() {
    aaa(); // todo: tests util module

    let port = setup().await;
    let response = http_response("", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"hello\":\"world\"}");
}

#[tokio::test]
async fn raw_path() {
    let port = setup().await;
    let response = http_response("/", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"hello\":\"world\"}");
}

#[tokio::test]
async fn api_root_as_empty() {
    let port = setup().await;
    let response = http_response("/api/v1", None, port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn api_root() {
    let port = setup().await;
    let response = http_response("/api/v1/", None, port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn api_home() {
    let port = setup().await;
    let response = http_response("/api/v1/home", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"key\":\"value\"}");
}

#[tokio::test]
async fn matcher_object_1() {
    let port = setup().await;
    let body = "{\"a\":{\"b\":{\"c\":\"1\"}}}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"apikey\":\"apivalue\"}");
}

#[tokio::test]
async fn matcher_object_2() {
    let port = setup().await;
    let body = "{\"a\":{\"b\":{\"c\":\"0\"}}}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"key\":\"value\"}");
}

#[tokio::test]
async fn matcher_object_3() {
    let port = setup().await;
    let body = "{\"a\":{\"b\":{\"c\":\"1\", \"d\": 0}}}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"apikey\":\"apivalue\"}");
}

#[tokio::test]
async fn matcher_data_type_insensitiveness() {
    let port = setup().await;
    let body = "{\"a\":{\"b\":{\"c\":1}}}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"apikey\":\"apivalue\"}");
}

#[tokio::test]
async fn matcher_object_missing() {
    let port = setup().await;
    let body = "{\"a\":{\"b\":{\"c\":\"2\"}}}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"key\":\"value\"}");
}

#[tokio::test]
async fn matcher_array() {
    let port = setup().await;
    let body = "{\"d\":[{},{},{\"e\":\"x=\"}]}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"apikey\":\"apivalue\"}");
}

#[tokio::test]
async fn matcher_array_missing() {
    let port = setup().await;
    let body = "{\"d\":[{\"e\":\"x=\"}]}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"key\":\"value\"}");
}

#[tokio::test]
async fn matcher_empty_value() {
    let port = setup().await;
    let body = "{\"f\":\"\"}";
    let response = http_response("/api/v1/some/path/w/matcher", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"apikey\":\"apivalue\"}");
}

#[tokio::test]
async fn error401() {
    let port = setup().await;
    let response = http_response("/api/v1/error/401", None, port).await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn error403() {
    let port = setup().await;
    let response = http_response("/api/v1/error/api-403", None, port).await;

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn port_env_arg_overwrites() {
    let port = u16::MAX;
    setup_with_port(port).await;

    let response = http_response("/", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"hello\":\"world\"}");
}

#[tokio::test]
async fn middleware_uri_path_handled() {
    let port = setup().await;
    let response = http_response("/middleware-test", None, port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"thisIs\":\"missedByConfigToml\"}");
}

#[tokio::test]
async fn middleware_uri_path_missed() {
    let port = setup().await;
    let response = http_response("/middleware-test/dummy", None, port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "");
}

#[tokio::test]
async fn middleware_body_handled() {
    let port = setup().await;
    let body = "{\"middleware\": \"isHere\"}";
    let response = http_response("/middleware-test/dummy", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"thisIs\":\"missedByConfigToml\"}");
}

#[tokio::test]
async fn middleware_body_missed() {
    let port = setup().await;
    let body = "{\"middleware\": \"isHere?\"}";
    let response = http_response("/middleware-test/dummy", Some(body), port).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "");
}

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

// utils
/// test initial setup: start up mock server
async fn setup() -> u16 {
    let port = dynamic_port();
    setup_with_port(port).await;
    port
}

/// test initial setup: start up mock server with specific port number
async fn setup_with_port(port: u16) {
    let _ = env::set_current_dir(TEST_WORKDIR);

    tokio::spawn(async move {
        let app = App::new(env_args(port), None, true).await;
        app.server.start().await
    });
    // wait for server started
    tokio::time::sleep(std::time::Duration::from_millis(400)).await;
}

/// env args for testing
fn env_args(port: u16) -> EnvArgs {
    let mut ret = EnvArgs::init_with_default();

    ret.config_file_path = Some(CONFIG_FILE_PATH.to_owned());
    ret.port = Some(port);
    ret.middleware_file_path = Some(MIDDLEWARE_FILE_PATH.to_owned());

    match ret.validate() {
        Ok(_) => ret,
        Err(_) => panic!("something wrong in env args"),
    }
}

/// select dynamic port randomly
fn dynamic_port() -> u16 {
    rand::rng().random_range(49152..=65535)
}

/// get http response from mock server
async fn http_response(uri_path: &str, body: Option<&str>, port: u16) -> Response<Incoming> {
    let uri: Uri = Uri::builder()
        .scheme("http")
        .authority(format!("127.0.0.1:{}", port.to_string()))
        .path_and_query(uri_path)
        .build()
        .unwrap();

    let host = uri.host().expect("uri has no host");
    let port = uri.port_u16().expect("some problem around port");
    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect(addr)
        .await
        .expect(format!("tcp connect failed with {}:{}", host, port).as_str());
    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await.unwrap();
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            log::error!("Connection failed: {:?}", err);
        }
    });

    let authority = uri.authority().unwrap().clone();

    let path = uri.path();
    let body = if body.is_none() {
        Empty::new().boxed()
    } else {
        Full::new(Bytes::from(body.unwrap().to_owned())).boxed()
    };
    let req = Request::builder()
        .uri(path)
        .header(hyper::header::HOST, authority.as_str())
        .body(body)
        .unwrap();

    sender.send_request(req).await.unwrap()
}

/// convert response body bytes to string
async fn response_body_str(response: Response<Incoming>) -> String {
    let body_bytes = response_body_bytes(response).await;
    let body_str = String::from_utf8(body_bytes.into()).unwrap();
    body_str
}

/// convert response body bytes to string
async fn response_body_bytes(response: Response<Incoming>) -> Bytes {
    response
        .into_body()
        .boxed()
        .collect()
        .await
        .unwrap()
        .to_bytes()
}
