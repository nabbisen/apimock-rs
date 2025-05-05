use hyper::{http::Error, Response};
use rhai::{Engine, Scope};

use super::types::BoxBody;

/// middleware
pub fn middleware(path: &str) -> Option<Result<Response<BoxBody>, Error>> {
    let path_for_middleware = path.to_owned();
    tokio::task::spawn_blocking(move || {
        let engine = Engine::new();
        let ast = engine
            .compile_file("middleware.rhai".into())
            .expect("todo1");
        let mut scope = Scope::new();
        scope.push("path", path_for_middleware);
        let middleware_response = engine
            .eval_ast_with_scope::<()>(&mut scope, &ast)
            .expect("todo2");
        println!("{:?}", middleware_response);
    });
    None
}
