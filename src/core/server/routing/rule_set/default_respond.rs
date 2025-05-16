use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct DefaultRespond {
    pub delay_response_milliseconds: Option<u16>,
}

impl DefaultRespond {
    pub fn print(&self) {
        if self.delay_response_milliseconds.is_some() {
            log::info!(
                "[[ delay_response_milliseconds ]] {}",
                self.delay_response_milliseconds.as_ref().unwrap()
            );
        }
    }
}
