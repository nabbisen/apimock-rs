use http_body_util::{BodyExt, Empty, Full};
use hyper::{
    body::{Bytes, Incoming},
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Request, Response, Uri,
};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

/// get http response from mock server - default
pub async fn http_response_default(url_path: &str, port: u16) -> Response<Incoming> {
    http_response(url_path, port, None, None).await
}

/// get http response from mock server - with headers condition
pub async fn http_response_headers_condition(
    url_path: &str,
    port: u16,
    headers: &HeaderMap<HeaderValue>,
) -> Response<Incoming> {
    http_response(url_path, port, Some(headers), None).await
}

/// get http response from mock server - with body condition
pub async fn http_response_body_condition(
    url_path: &str,
    port: u16,
    headers: Option<&HeaderMap<HeaderValue>>,
    body: &str,
) -> Response<Incoming> {
    http_response(url_path, port, headers, Some(body)).await
}

/// get http response from mock server - with body condition as json
pub async fn http_response_json_body_condition(
    url_path: &str,
    port: u16,
    body: &str,
) -> Response<Incoming> {
    let headers: HeaderMap = [(CONTENT_TYPE, "application/json")]
        .into_iter()
        .map(|(k, v)| (k, HeaderValue::from_static(v)))
        .collect();

    http_response_body_condition(url_path, port, Some(&headers), body).await
}

/// get http response from mock server
async fn http_response(
    url_path: &str,
    port: u16,
    headers: Option<&HeaderMap<HeaderValue>>,
    body: Option<&str>,
) -> Response<Incoming> {
    let url: Uri = Uri::builder()
        .scheme("http")
        .authority(format!("127.0.0.1:{}", port.to_string()))
        .path_and_query(url_path)
        .build()
        .unwrap();

    let host = url.host().expect("url has no host");
    let port = url.port_u16().expect("some problem around port");
    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect(addr)
        .await
        .expect(&format!("tcp connect failed with {}:{}", host, port));
    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await.unwrap();
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            log::error!("connection failed: {:?}", err);
        }
    });

    let authority = url.authority().unwrap().clone();

    let path = url.path();
    let body = if body.is_none() {
        Empty::new().boxed()
    } else {
        Full::new(Bytes::from(body.unwrap().to_owned())).boxed()
    };
    let mut builder = Request::builder()
        .uri(path)
        .header(hyper::header::HOST, authority.as_str());
    if let Some(headers) = headers {
        for (header_key, header_value) in headers.iter() {
            builder = builder.header(header_key, header_value);
        }
    }
    let req = builder.body(body).expect("failed to create http request");

    sender.send_request(req).await.unwrap()
}

/// convert response body bytes to string
pub async fn response_body_str(response: Response<Incoming>) -> String {
    let body_bytes = response_body_bytes(response).await;
    let body_str = String::from_utf8(body_bytes.into()).unwrap();
    body_str
}

/// convert response body bytes to string
pub async fn response_body_bytes(response: Response<Incoming>) -> Bytes {
    response
        .into_body()
        .boxed()
        .collect()
        .await
        .unwrap()
        .to_bytes()
}
