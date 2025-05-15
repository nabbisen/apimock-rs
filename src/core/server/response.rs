use hyper::{
    header::{
        HeaderValue, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
        ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE,
    },
    http::response::Builder,
};

pub mod error_response;
pub mod file_response;
pub mod text_response;
mod util;

pub fn default_builder() -> Builder {
    let builder = hyper::Response::builder()
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"))
        .header(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"))
        .header(
            ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("GET, POST, OPTIONS"),
        );
    builder
}

/// response base on json response
pub fn json_builder() -> Builder {
    default_builder().header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
}

// /// response on `data_dir` paths
// pub fn static_path_response(
//     path_config: &PathConfig,
//     headers: Option<&HashMap<HeaderId, HeaderConfig>>,
//     request_uri_path: &str,
// ) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
//     if let Some(_) = &path_config.data_src {
//         return static_path_data_src_reponse(path_config, headers, request_uri_path);
//     }

//     if let Some(data_text) = &path_config.data_text {
//         return json_response_base(path_config.headers.as_ref(), headers)
//             .status(path_config.code)
//             .body(Full::new(Bytes::from(data_text.to_owned())).boxed());
//     }

//     response_base(path_config.headers.as_ref(), headers)
//         .status(path_config.code)
//         .body(Empty::new().boxed())
// }

// /// json file on `data_src` response on `data_dir` paths
// fn static_path_data_src_reponse(
//     path_config: &PathConfig,
//     headers: Option<&HashMap<HeaderId, HeaderConfig>>,
//     request_uri_path: &str,
// ) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
//     match path_config.data_src.as_ref() {
//         Some(data_src) => {
//             file_to_response(data_src.as_str(), path_config.headers.as_ref(), headers)
//         }
//         None => internal_server_error_response(
//             format!("{}: data_src is missing", request_uri_path).as_str(),
//         ),
//     }
// }

// /// response base on any
// pub fn response_base(
//     path_headers: Option<&Vec<String>>,
//     headers: Option<&HashMap<HeaderId, HeaderConfig>>,
// ) -> Builder {
//     let mut ret = hyper::Response::builder()
//         .header(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"))
//         .header(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"))
//         .header(
//             ACCESS_CONTROL_ALLOW_METHODS,
//             HeaderValue::from_static("GET, POST, OPTIONS"),
//         );
//     if let Some(path_headers) = path_headers {
//         let headers = headers.clone().unwrap();
//         for path_header in path_headers {
//             let header = headers.get(path_header).unwrap();
//             ret = ret.header(header.key.as_str(), header.value.clone().unwrap().as_str());
//         }
//     }
//     ret
// }
