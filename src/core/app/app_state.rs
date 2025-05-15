

use crate::core::{config::Config, server::middleware::Middleware};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub middleware: Option<Middleware>,
}
