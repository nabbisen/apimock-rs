use serde::Deserialize;

use std::collections::HashMap;

use crate::core::server::{
    response::{file_response::FileResponse, text_response::text_response},
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
    pub async fn response(&self) -> Result<hyper::Response<BoxBody>, hyper::http::Error> {
        if let Some(delay_response_milliseconds) = self.delay_response_milliseconds {
            delay_response(delay_response_milliseconds).await;
        }

        match self.response_type {
            Some(ResponseType::Text) => {
                text_response(self.content.as_str(), None, self.headers.as_ref())
            }
            Some(ResponseType::File) | None => {
                FileResponse::new(self.content.as_str(), self.headers.as_ref())
                    .file_content_response()
            }
        }
    }
}
