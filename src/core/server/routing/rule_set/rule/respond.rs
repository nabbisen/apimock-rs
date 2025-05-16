use serde::Deserialize;
use util::full_file_path;

use std::collections::HashMap;

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
    pub headers: Option<HashMap<String, Option<String>>>,
    pub code: Option<u16>,
    pub content: String,
    pub delay_response_milliseconds: Option<u16>,
}

impl Respond {
    pub async fn response(
        &self,
        path_prefix: Option<String>,
    ) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
        if let Some(delay_response_milliseconds) = self.delay_response_milliseconds {
            delay_response(delay_response_milliseconds).await;
        }

        match self.response_type {
            Some(ResponseType::Text) => {
                text_response(self.content.as_str(), None, self.headers.as_ref())
            }
            Some(ResponseType::File) | None => {
                let path_prefix = if let Some(path_prefix) = path_prefix {
                    path_prefix
                } else {
                    String::new()
                };
                let file_path = full_file_path(self.content.as_str(), path_prefix.as_str());
                if file_path.is_none() {
                    log::error!(
                        "{} (prefix = {}) is missing",
                        self.content.as_str(),
                        path_prefix.as_str()
                    );
                    return internal_server_error_response("failed to get response file");
                }
                FileResponse::new(file_path.unwrap().as_str(), self.headers.as_ref())
                    .file_content_response()
            }
        }
    }

    pub fn print(&self) {
        // todo: print()
    }
}
