use console::style;
use http_body_util::BodyExt;
use hyper::body::Incoming;
use hyper::http::request::Parts;
use serde_json::Value;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::{
    config::log_config::verbose_config::VerboseConfig,
    util::http::{content_type_is_application_json, normalize_url_path},
};

#[derive(Debug)]
pub struct ParsedRequest {
    pub url_path: String,
    pub component_parts: Parts,
    pub body_json: Option<Value>,
}

impl ParsedRequest {
    pub async fn from(request: hyper::Request<Incoming>) -> Result<Self, String> {
        let (component_parts, body) = request.into_parts();

        let body_bytes = match body.boxed().collect().await {
            Ok(x) => Some(x.to_bytes()),
            Err(err) => {
                log::warn!("failed to collect request incoming body: {}", err);
                None
            }
        };

        let has_body = body_bytes.is_some() && !body_bytes.as_ref().unwrap().is_empty();

        let mut body_json: Option<Value> = None;
        if has_body {
            let raw_body_json = serde_json::from_slice::<Option<Value>>(&body_bytes.unwrap());

            let _ = match content_type_is_application_json(&component_parts.headers) {
                // case application/json: get json body
                Some(x) if x && raw_body_json.is_err() => {
                    return Err(format!(
                        "failed to get json value from request body: {}",
                        raw_body_json.unwrap_err()
                    ));
                }
                // todo: support other types than json (such as form value) in the future
                Some(x) if !x => {
                    log::warn!("request has body but its content-type is not application/json")
                }
                None => log::warn!("request has body but doesn't have content-type"),
                _ => (),
            };

            if raw_body_json.is_ok() {
                body_json = raw_body_json.unwrap();
            }
        }

        let url_path = normalize_url_path(component_parts.uri.path(), None);

        Ok(ParsedRequest {
            url_path,
            component_parts,
            body_json,
        })
    }

    /// print out logs
    pub fn capture_in_log(&self, verbose: VerboseConfig) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let hours = (now / 3600) % 24;
        let minutes = (now / 60) % 60;
        let seconds = now % 60;
        let timestamp = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

        // url and timestamp (base)
        let mut printed = format!(
            "<- {} (request got at {} UTC)",
            style(self.url_path.as_str()).yellow(),
            timestamp
        );

        // headers
        if verbose.header || verbose.body {
            printed.push_str("\n");
        }
        if verbose.header {
            let headers = self
                .component_parts
                .headers
                .iter()
                .map(|x| format!("\n{}: {}", x.0, x.1.to_str().unwrap()))
                .collect::<String>();
            let printed_headers = format!(
                "({:?}, {}){}",
                self.component_parts.version, self.component_parts.method, headers
            );
            printed = format!("{}{}", printed, style(printed_headers).magenta());
        }
        // body (json params)
        let mut is_verbose_body = false;
        if verbose.body {
            if let Some(request_body_json_value) = &self.body_json {
                let body_str = request_body_json_value.to_string();
                printed = format!("{}\n{}", printed, style(body_str).green());
                is_verbose_body = true;
            }
        }
        if verbose.header || is_verbose_body {
            printed.push_str("\n");
        }

        log::info!("{}", printed);
    }
}
