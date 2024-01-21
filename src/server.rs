use std::collections::HashMap;
use warp::{Filter};
use warp::reply::{json as warp_json};
use console::style;

use crate::config::config;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let config = config();
    println!("{:?}", style(config).red()); // todo

    // /
    let root_path = warp::path::end().map(|| "Hello, world from ROOT !");
    // /hi
    let hi = warp::path("hi").map(|| "Hello, world !");
    // /hello/:string
    let hello = warp::path("hello")
        .and(warp::path::param())
        .map(|name: String| format!("Hello, {} !", name));
    // /sum/:u32/:u32
    let sum = warp::path!("sum" / u32 / u32).map(|a, b| format!("{} + {} = {}", a, b, a + b));
    // /json
    let json = warp::path("json").map(|| {
        let json_body = warp_json(&HashMap::from([
            ("id", 1),
            ("name", 2),
        ]));
        json_body
    });
    let paths = root_path.or(hi).or(hello).or(sum).or(json);

    let get_routes = warp::get().and(paths);
    let post_routes = warp::post().and(paths);
    
    let routes = get_routes.or(post_routes);
    routes
}