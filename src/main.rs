#[macro_use]
extern crate log;
extern crate env_logger;
extern crate iron;

use iron::prelude::*;
use iron::status;
use std::env;

fn main() {
    env_logger::init().unwrap();
    info!("starting up");

    fn hello_world(request: &mut Request) -> IronResult<Response> {
        info!("Request: {}", request.url);
        Ok(Response::with((status::Ok, "Hello World!?")))
    }

    match env::var("PORT") {
        Ok(port) => println!("Port: {}", port),
        Err(e) => println!("Couldn't read LANG ({})", e),
    };
    let port: String = env::var("PORT").unwrap();
    info!("Port: {}", port);
    Iron::new(hello_world)
        .http(format!("0.0.0.0:{}", port))
        .unwrap();
}
