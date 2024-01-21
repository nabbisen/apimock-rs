use std::net::SocketAddr;
use std::str::FromStr;

mod config;
mod server;
use crate::config::config;
use crate::server::routes;

pub const LISTEN_ADDR: &str = "127.0.0.1";
pub const LISTEN_PORT: u16 = 3001;
// const PATHS: [(&str, usize); 3] = [
//     ("hi", "Hello, world!"),
//     ("hello", "Hello, {}!"),
//     ("sum", "{}"),
// ];
pub const CONFIG_FILENAME: &str = "json-responder.toml";

#[tokio::main]
async fn main() {
    let config = config();
    let routes = routes(&config);

    let addr_port = format!("{}:{}", LISTEN_ADDR, config.port.to_string());
    let listener = SocketAddr::from_str(addr_port.as_str()).unwrap();

    println!("Start listening on {} ...", addr_port);
    warp::serve(routes)
        .run(listener)
        .await;
}
