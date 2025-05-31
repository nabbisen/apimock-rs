use http_body_util::BodyExt;
use hyper::{
    body::{Bytes, Incoming},
    Response,
};

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
