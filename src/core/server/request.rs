use console::style;
use hyper::http::request::Parts;
use serde_json::Value;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::config::VerboseConfig;

/// print out logs
pub fn capture_in_log(
    request_uri_path: &str,
    request_header: &Parts,
    request_body_json_value: Option<&Value>,
    verbose: VerboseConfig,
) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let hours = (now / 3600) % 24;
    let minutes = (now / 60) % 60;
    let seconds = now % 60;
    let timestamp = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

    // uri and timestamp (base)
    let mut printed = format!(
        "<- {} (request got at {} UTC)",
        style(request_uri_path).yellow(),
        timestamp
    );

    // headers
    if verbose.header || verbose.body {
        printed.push_str("\n");
    }
    if verbose.header {
        let headers = request_header
            .headers
            .iter()
            .map(|x| format!("\n{}: {}", x.0, x.1.to_str().unwrap()))
            .collect::<String>();
        let printed_headers = format!(
            "({:?}, {}){}",
            request_header.version, request_header.method, headers
        );
        printed = format!("{}{}", printed, style(printed_headers).magenta());
    }
    // body (json params)
    let mut is_verbose_body = false;
    if verbose.body {
        if let Some(request_body_json_value) = request_body_json_value {
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
