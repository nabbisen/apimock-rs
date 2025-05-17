use console::style;
use hyper::StatusCode;
use serde::Deserialize;
use util::full_file_path;

use std::{collections::HashMap, path::Path};

mod util;

use crate::core::server::{
    response::{
        error_response::internal_server_error_response, file_response::FileResponse,
        status_code_response::status_code_response_with_message, text_response::text_response,
    },
    types::BoxBody,
    util::delay_response,
};

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ResponseType {
    File,
    Text,
    CustomCode,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Respond {
    pub response_type: Option<ResponseType>,
    pub content: Option<String>,
    pub code: Option<u16>,
    #[serde(skip)]
    pub status_code: Option<StatusCode>,
    pub headers: Option<HashMap<String, Option<String>>>,
    pub delay_response_milliseconds: Option<u16>,
}

impl Respond {
    /// generate response
    pub async fn response(
        &self,
        dir_prefix: &str,
    ) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
        if let Some(delay_response_milliseconds) = self.delay_response_milliseconds {
            delay_response(delay_response_milliseconds).await;
        }

        match self.response_type {
            Some(ResponseType::Text) => text_response(
                self.content.clone().unwrap_or_default().as_str(),
                None,
                self.headers.as_ref(),
            ),
            Some(ResponseType::CustomCode) => {
                if let Some(status_code) = self.status_code.as_ref() {
                    status_code_response_with_message(
                        status_code,
                        self.content.clone().unwrap_or_default().as_str(),
                    )
                } else {
                    internal_server_error_response("respond status code is missing")
                }
            }
            Some(ResponseType::File) | None => {
                let file_path = full_file_path(
                    self.content.clone().unwrap_or_default().as_str(),
                    dir_prefix,
                );
                if file_path.is_none() {
                    log::error!(
                        "{} (prefix = {}) is missing",
                        self.content.clone().unwrap_or_default().as_str(),
                        dir_prefix
                    );
                    return internal_server_error_response("failed to get response file");
                }
                FileResponse::new(file_path.unwrap().as_str(), self.headers.as_ref())
                    .file_content_response()
            }
        }
    }

    /// validate
    pub fn validate(&self, dir_prefix: &str, rule_idx: usize, rule_set_idx: usize) -> bool {
        let validate = if let Some(content) = self.content.as_ref() {
            content_validate(
                content.as_str(),
                self.response_type.as_ref(),
                dir_prefix,
                rule_idx,
                rule_set_idx,
            )
        } else {
            code_validate(self.status_code.as_ref(), rule_idx, rule_set_idx)
        };

        validate
    }
}

impl std::fmt::Display for Respond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "  [respond] ");
        let _ = match self.response_type {
            Some(ResponseType::Text) => writeln!(f, "(text)"),
            Some(ResponseType::CustomCode) => {
                writeln!(f, "{} Statsu code", self.status_code.unwrap_or_default())
            }
            Some(ResponseType::File) | None => {
                if let Some(content) = self.content.as_ref() {
                    let _ = write!(f, "{}", style(content).green());
                }
                Ok(())
            }
        };
        let _ = writeln!(f, "");

        Ok(())
    }
}

/// validate on content with response type
fn content_validate(
    content: &str,
    response_type: Option<&ResponseType>,
    dir_prefix: &str,
    rule_idx: usize,
    rule_set_idx: usize,
) -> bool {
    match response_type {
        Some(ResponseType::Text) | Some(ResponseType::CustomCode) => true,
        Some(ResponseType::File) | None => {
            let ret = Path::new(dir_prefix).join(content).exists();
            if !ret {
                let p = if !dir_prefix.is_empty() {
                    format!("{}/{}", dir_prefix, content)
                } else {
                    String::from(content)
                };
                log::error!(
                    "`{}` does not exist (rule #{} in rule set #{})",
                    p.as_str(),
                    rule_idx + 1,
                    rule_set_idx + 1
                );
            }
            ret
        }
    }
}

fn code_validate(code: Option<&StatusCode>, rule_idx: usize, rule_set_idx: usize) -> bool {
    let ret = code.is_some();
    if !ret {
        log::error!(
            "status code is required (rule #{} in rule set #{})",
            rule_idx + 1,
            rule_set_idx + 1
        );
    }
    ret
}
