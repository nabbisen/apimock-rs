use rhai::{Dynamic, Engine, Scope, AST};

/// return string if middleware returns
pub fn handle(request_uri_path: &str, engine: &Engine, ast: &AST) -> Option<String> {
    let mut scope = Scope::new();
    scope.push("uri_path", request_uri_path.to_owned());

    let middleware_response = engine
        .eval_ast_with_scope::<Dynamic>(&mut scope, ast)
        // todo: error msg
        .expect("todo2");
    if middleware_response.type_name() == "string" {
        return Some(middleware_response.to_string());
    }

    None
}
