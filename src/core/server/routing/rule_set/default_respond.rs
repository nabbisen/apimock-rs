use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct DefaultRespond {
    pub delay_response_milliseconds: Option<u16>,
}
