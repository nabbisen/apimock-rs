use respond::Respond;
use serde::Deserialize;
use when::When;

pub mod respond;
mod types;
pub mod when;

#[derive(Clone, Deserialize, Debug)]
pub struct Rule {
    pub when: When,
    pub respond: Respond,
}

// // response with static paths routing
// if let Some(paths) = &config.paths {
//     if let Some(x) = handle_static_path(
//         request_uri_path.as_str(),
//         request_body_json_value.as_ref(),
//         paths,
//         config.paths_jsonpath_patterns.as_ref(),
//         config.headers.as_ref(),
//     )
//     .await
//     {
//         return x;
//     }
// }

// // response with dynamic paths routing
// if let Some(dyn_data_dir) = &config.dyn_data_dir.clone() {
//     handle_dyn_path(request_uri_path.as_str(), dyn_data_dir.as_str())
// } else {
//     not_found_response()
// }

// /// handle on `data_dir_query_path` config
// fn handle_data_dir_query_path(config: &Config, request_uri_path: &str) -> Option<String> {
//     if request_uri_path == "" || config.data_dir_query_path.is_none() {
//         return None;
//     }

//     let data_dir_query_path = config.data_dir_query_path.clone().unwrap();

//     let stripped = request_uri_path
//         .strip_prefix("/")
//         .unwrap()
//         .strip_prefix(data_dir_query_path.as_str());
//     match stripped {
//         Some(x) => return Some(x.to_owned()),
//         None => return None,
//     }
// }

// /// handle on `data_dir` paths (static json responses)
// async fn handle_static_path(
//     request_uri_path: &str,
//     request_body_json_value: Option<&Value>,
//     path_configs: &ConfigUrlPaths,
//     paths_jsonpath_patterns: Option<
//         &HashMap<String, HashMap<String, Vec<JsonpathMatchingPattern>>>,
//     >,
//     headers: Option<&HashMap<HeaderId, HeaderConfig>>,
// ) -> Option<Result<Response<BoxBody>, hyper::http::Error>> {
//     let path_config_hashmap = path_configs
//         .iter()
//         .find(|(key, _)| key.as_str() == request_uri_path);
//     if let Some(path_config_hashmap) = path_config_hashmap {
//         let mut path_config = path_config_hashmap.1.clone();

//         if let Some(request_body_json_value) = request_body_json_value {
//             match_jsonpath_patterns(
//                 &mut path_config,
//                 request_uri_path,
//                 request_body_json_value,
//                 paths_jsonpath_patterns,
//             )
//             .await;
//         }

//         delay_response(path_config.response_wait_more_millis).await;

//         let response = Some(static_path_response(
//             &path_config,
//             headers,
//             request_uri_path,
//         ));
//         return response;
//     }

//     None
// }
