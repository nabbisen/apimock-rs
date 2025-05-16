use console::style;
use serde::Deserialize;
use util::full_file_path;

use std::{collections::HashMap, path::Path};

mod util;

use crate::core::server::{
    response::{
        error_response::internal_server_error_response, file_response::FileResponse,
        text_response::text_response,
    },
    types::BoxBody,
    util::delay_response,
};

#[derive(Clone, Deserialize, Debug)]
pub enum ResponseType {
    File,
    Text,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Respond {
    pub response_type: Option<ResponseType>,
    pub content: String,
    pub code: Option<u16>,
    pub headers: Option<HashMap<String, Option<String>>>,
    pub delay_response_milliseconds: Option<u16>,
}

impl std::fmt::Display for Respond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = match self.response_type {
            Some(ResponseType::Text) => writeln!(f, "(text)"),
            Some(ResponseType::File) | None => {
                writeln!(f, "{}", style(self.content.as_str()).green())
            }
        };

        Ok(())
    }
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
            Some(ResponseType::Text) => {
                text_response(self.content.as_str(), None, self.headers.as_ref())
            }
            Some(ResponseType::File) | None => {
                let file_path = full_file_path(self.content.as_str(), dir_prefix);
                if file_path.is_none() {
                    log::error!(
                        "{} (prefix = {}) is missing",
                        self.content.as_str(),
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
    pub fn validate(&self, dir_prefix: &str) -> bool {
        let content_validate: bool = content_validate(
            self.content.as_str(),
            self.response_type.as_ref(),
            dir_prefix,
        );
        let code_validate = self.code.is_none() || code_validate(self.code.as_ref().unwrap());

        content_validate && code_validate
    }
}

/// validate on content with response type
fn content_validate(content: &str, response_type: Option<&ResponseType>, dir_prefix: &str) -> bool {
    match response_type {
        Some(ResponseType::Text) => true,
        Some(ResponseType::File) | None => {
            let ret = Path::new(dir_prefix).join(content).exists();
            if !ret {
                let p = if !dir_prefix.is_empty() {
                    format!("{}/{}", dir_prefix, content)
                } else {
                    String::from(content)
                };
                log::error!("`{}` does not exist", p.as_str());
            }
            ret
        }
    }
}

/// validate on http status code
fn code_validate(code: &u16) -> bool {
    let ret = (100..=599).contains(code);
    if !ret {
        log::error!("{} is out of http status code range", code);
    }
    ret
}
