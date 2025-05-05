use std::sync::Arc;

use rhai::{Engine, AST};

use super::{config::Config, util::args_option_value};

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

/// app middleware file path
///
/// - if specified with command-line option, use it
/// - else miss it
pub fn middleware_filepath() -> String {
    let option_names: Vec<&str> = vec!["--middleware"];
    args_option_value(&option_names)
}
