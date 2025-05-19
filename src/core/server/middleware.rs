use std::{path::Path, sync::Arc};

use rhai::{serde::to_dynamic, Dynamic, Engine, Scope, AST};
use serde_json::Value;

#[derive(Clone)]
pub struct Middleware {
    pub engine: Arc<Engine>,
    pub file_path: String,
    pub ast: AST,
}

impl Middleware {
    pub fn new(file_path: &str) -> Result<Self, String> {
        if !Path::new(file_path).exists() {
            return Err(format!("middleware file path must be wrong: {}", file_path));
        }

        let engine = Engine::new();
        // todo: watch source file change - `notify` crate ?
        let ast = engine.compile_file(file_path.into()).expect(
            format!(
                "failed to compile middleware file to get ast: {}",
                file_path
            )
            .as_str(),
        );

        let middleware = Middleware {
            engine: Arc::new(engine),
            file_path: file_path.to_owned(),
            ast,
        };

        Ok(middleware)
    }

    /// return string if middleware returns
    pub fn handle(
        &self,
        request_url_path: &str,
        request_body_json_value: Option<&Value>,
    ) -> Option<String> {
        let mut scope = Scope::new();
        scope.push("url_path", request_url_path.to_owned());
        if let Some(request_body_json_value) = request_body_json_value {
            scope.push(
                "body",
                to_dynamic(request_body_json_value)
                    .expect("failed to request body to dynamic for middleware"),
            );
        }

        let middleware_response = self
            .engine
            .eval_ast_with_scope::<Dynamic>(&mut scope, &self.ast)
            .expect("failed to evaluate middleware");
        if middleware_response.type_name() == "string" {
            return Some(middleware_response.to_string());
        }

        None
    }
}
