use hyper::StatusCode;

use crate::{
    constant::root_config_dir,
    util::{
        http::{http_response_default, response_body_str},
        test_setup::TestSetup,
    },
};

#[tokio::test]
async fn matches_dyn_data_dir_csv_jsonpath_key() {
    let port = setup().await;
    let response = http_response_default("/respond/csv/records/jsonpath", port).await;

    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_str = response_body_str(response).await;
    assert_eq!(body_str.as_str(), "{\"a\":{\"b\":{\"c\":[{\"fieldA\":\"1\",\"fieldB\":\"2\",\"fieldC\":\"3\"},{\"fieldA\":\"a\",\"fieldB\":\"b\",\"fieldC\":\"c\"},{\"fieldA\":\"#\",\"fieldB\":\"\\\\,\",\"fieldC\":\"!!!\"}]}}}");
}

/// internal setup fn
async fn setup() -> u16 {
    let test_setup = TestSetup::default_with_root_config_dir(root_config_dir::RULE_RESPOND);
    let port = test_setup.launch().await;
    port
}
