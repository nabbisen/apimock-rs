use std::sync::Arc;

use rhai::{Engine, AST};

use crate::MIDDLEWARE_FILEPATH_OPTION_NAMES;

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
    args_option_value(&MIDDLEWARE_FILEPATH_OPTION_NAMES.to_vec())
}
