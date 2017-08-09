#[macro_use]
extern crate log;
extern crate env_logger;
extern crate iron;
extern crate tera;
extern crate router;
extern crate iron_tera;
extern crate staticfile;
extern crate mount;

use iron::prelude::*;
use iron::status;
use router::Router;
use tera::Context;
use iron_tera::{Template, TeraEngine, TemplateMode};
use staticfile::Static;
use std::env;
use mount::Mount;

fn main() {
    env_logger::init().unwrap();
    info!("starting up");

    let default_port = String::from("8000");
    let port: String = env::var("PORT").unwrap_or(default_port);
    info!("Port: {}", port);

    let mut router = Router::new();
    router.get("/", hello_world, "hello_world");

    let mut mount = Mount::new();
    mount.mount("/static", Static::new("static/"));
    mount.mount("/", router);

    let mut chain = Chain::new(mount);
    let teng = TeraEngine::new("templates/**/*");
    chain.link_after(teng);

    Iron::new(chain).http(format!("0.0.0.0:{}", port)).unwrap();
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let mut context = Context::new();
    context.add("my_var", &"Thing"); // comment out to see alternate thing

    resp.set_mut(Template::new(
        "base.html",
        TemplateMode::from_context(context),
    )).set_mut(status::Ok);
    Ok(resp)
}
