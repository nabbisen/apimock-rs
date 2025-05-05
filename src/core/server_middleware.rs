use rhai::{serde::to_dynamic, Dynamic, Engine, Scope, AST};
use serde_json::Value;

/// return string if middleware returns
pub fn handle(
    request_uri_path: &str,
    request_body_json_value: Option<&Value>,
    engine: &Engine,
    ast: &AST,
) -> Option<String> {
    let mut scope = Scope::new();
    scope.push("uri_path", request_uri_path.to_owned());
    if let Some(request_body_json_value) = request_body_json_value {
        scope.push(
            "body",
            to_dynamic(request_body_json_value)
                .expect("failed to request body to dynamic for middleware"),
        );
    }

    let middleware_response = engine
        .eval_ast_with_scope::<Dynamic>(&mut scope, ast)
        // todo: error msg
        .expect("todo2");
    if middleware_response.type_name() == "string" {
        return Some(middleware_response.to_string());
    }

    None
}
