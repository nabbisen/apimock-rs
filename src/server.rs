use std::collections::HashMap;
use warp::{Filter, Reply, Rejection};
use warp::reply::{json as warp_json};
// use json5;
use console::style;

use crate::config::Config;

pub fn routes(config: &Config) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    println!("{:?}", style(config).red()); // todo

    // match &config.always {
    //     Some(json) => {
    //         let always = json5::from_str(json).expect("Invalid always json str");
    //         return warp::any().map(|| always)
    //     },
    //     _ => (),
    // }

    // todo: root
    let root_path = warp::path::end().map(|| "Hello, world from ROOT !");
    let mut paths = HashMap::<String, impl Filter<Extract = impl Reply, Error = Rejection> + Clone>::new();
    match &config.paths {
        Some(hash_map) => {
            for (key, value) in hash_map {
                let path = warp::path(key).map(|| value);
                paths.insert(path);
            }
        },
        _ => (),
    }
    let get_routes = warp::get().and(paths);
    let post_routes = warp::post().and(paths);
    let routes = get_routes.or(post_routes);
    routes

    // // /
    // let root_path = warp::path::end().map(|| "Hello, world from ROOT !");
    // // /hi
    // let hi = warp::path("hi").map(|| "Hello, world !");
    // // /hello/:string
    // let hello = warp::path("hello")
    //     .and(warp::path::param())
    //     .map(|name: String| format!("Hello, {} !", name));
    // // /sum/:u32/:u32
    // let sum = warp::path!("sum" / u32 / u32).map(|a, b| format!("{} + {} = {}", a, b, a + b));
    // // /json
    // let json = warp::path("json").map(|| {
    //     let json_body = warp_json(&HashMap::from([
    //         ("id", 1),
    //         ("name", 2),
    //     ]));
    //     json_body
    // });
    // let paths = root_path.or(hi).or(hello).or(sum).or(json);

    // let get_routes = warp::get().and(paths);
    // let post_routes = warp::post().and(paths);
    
    // let routes = get_routes.or(post_routes);
    // routes
}
