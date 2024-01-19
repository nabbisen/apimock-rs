use std::net::SocketAddr;
use std::str::FromStr;
use warp::Filter;

const LISTEN_ADDR: &str = "127.0.0.1";
const LISTEN_PORT: &str = "3001";
// const PATHS: [(&str, usize); 3] = [
//     ("hi", "Hello, world!"),
//     ("hello", "Hello, {}!"),
//     ("sum", "{}"),
// ];

#[tokio::main]
async fn main() {
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
    let paths = root_path.or(hi).or(hello).or(sum);

    let get_routes = warp::get().and(paths);
    let post_routes = warp::post().and(paths);
    let routes = get_routes.or(post_routes);

    let addr_port = format!("{}:{}", LISTEN_ADDR, LISTEN_PORT);
    let listener = SocketAddr::from_str(addr_port.as_str()).unwrap();
    println!("Start listening on {} ...", addr_port);

    warp::serve(routes)
        .run(listener)
        .await;
}
