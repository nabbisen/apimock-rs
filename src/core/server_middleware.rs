use hyper::{http::Error, Response};
use rhai::{Engine, Scope, AST};

use super::types::BoxBody;

/// middleware
pub fn middleware(
    path: &str,
    engine: &Engine,
    ast: &AST,
) -> Option<Result<Response<BoxBody>, Error>> {
    let mut scope = Scope::new();
    // todo: args
    scope.push("path", path.to_owned());

    let middleware_response = engine
        .eval_ast_with_scope::<()>(&mut scope, ast)
        .expect("todo2");
    println!("{:?}", middleware_response);

    // todo: return response if exists
    None
}
