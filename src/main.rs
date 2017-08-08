#[macro_use]
extern crate log;
extern crate env_logger;
extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    env_logger::init().unwrap();
    info!("starting up");

    fn hello_world(request: &mut Request) -> IronResult<Response> {
        info!("Request: {}", request.url);
        Ok(Response::with((status::Ok, "Hello World!?")))
    }

    info!("Pre-start!");
    Iron::new(hello_world).http("localhost:3000").unwrap();
    info!("On 3000");
}
