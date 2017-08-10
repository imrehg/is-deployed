extern crate is_deployed;
extern crate env_logger;

use std::env;
use is_deployed::start_server;

fn main() {
    env_logger::init().unwrap();

    let default_host = String::from("0.0.0.0");
    let host: String = env::var("HOST").unwrap_or(default_host);

    let default_port = String::from("8000");
    let port: String = env::var("PORT").unwrap_or(default_port);

    start_server(&host, &port);
}
