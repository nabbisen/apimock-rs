use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct RuleSetDefault {
    pub delay_response_milliseconds: Option<u16>,
}
