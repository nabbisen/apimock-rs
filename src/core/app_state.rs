use std::{path::Path, sync::Arc};

use rhai::{Engine, AST};

use crate::{DEFAULT_MIDDLEWARE_FILEPATH, MIDDLEWARE_FILEPATH_OPTION_NAMES};

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
/// - if specified in arguments, use it
/// - else if default file exists, use it
/// - else miss it
pub fn middleware_filepath() -> String {
    let option_value = args_option_value(&MIDDLEWARE_FILEPATH_OPTION_NAMES.to_vec());
    if !option_value.is_empty() {
        return option_value;
    }

    if Path::new(DEFAULT_MIDDLEWARE_FILEPATH).exists() {
        return DEFAULT_MIDDLEWARE_FILEPATH.to_owned();
    }

    String::new()
}
