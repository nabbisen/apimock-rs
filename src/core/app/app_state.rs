use std::sync::Arc;

use rhai::{Engine, AST};

use crate::core::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub middleware: Option<Middleware>,
}

#[derive(Clone)]
pub struct Middleware {
    pub engine: Arc<Engine>,
    pub filepath: String,
    pub ast: AST,
}
