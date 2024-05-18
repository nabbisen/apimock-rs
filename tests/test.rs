use apimock::{start_server, config::DEFAULT_LISTEN_PORT};

use hyper::{Body, Client, Request, StatusCode, Uri};

#[tokio::test]
async fn test_server_response() {
    tokio::spawn(start_server());
    // wait for server started
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let client = Client::new();

    let uri = Uri::builder()
        .scheme("http")
        .authority("127.0.0.1".to_string() + ":" + &DEFAULT_LISTEN_PORT.to_string())
        .path_and_query("/api/v1/")
        .build()
        .unwrap();

    let request = Request::builder()
        .uri(uri)
        .method("POST")
        .header("Content-Type", "text/plain")
        .body(Body::from(""))
        .unwrap();
    let response = client.request(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
