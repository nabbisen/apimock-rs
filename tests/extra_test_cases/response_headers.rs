use hyper::{header::HeaderValue, StatusCode};

use crate::util::{http::http_response_default, test_setup::TestSetup};

const HEADERS_RESPONSE_MUST_HAVE: &[(&str, Option<&str>)] = &[
    ("date", None),
    ("content-length", None),
    ("access-control-allow-origin", None),
    ("cache-control", Some("no-store")),
    ("access-control-allow-credentials", Some("true")),
    ("access-control-allow-headers", Some("*")),
    (
        "access-control-allow-methods",
        Some("GET, POST, PUT, DELETE, OPTIONS"),
    ),
    ("x-content-type-options", Some("nosniff")),
    ("connection", Some("keep-alive")),
];

#[tokio::test]
async fn http_headers_response_must_have() {
    let port = TestSetup::default().launch().await;

    let response = http_response_default("/root1", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    for (header_key, header_value) in HEADERS_RESPONSE_MUST_HAVE {
        let header = response
            .headers()
            .get(*header_key)
            .expect(&format!("failed to get header: {}", header_key));
        if let Some(header_value) = header_value {
            assert_eq!(header, HeaderValue::from_static(header_value),);
        }
    }
}
