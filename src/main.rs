extern crate chrono;
extern crate env_logger;
extern crate iron;
extern crate logger;
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate base64;
extern crate jsonwebtoken as jwt;
extern crate postgres;
extern crate serde_json;
extern crate uuid;
extern crate validator;
#[macro_use]
extern crate validator_derive;

mod models;
mod handlers;
mod routes;
mod utils;

use iron::Iron;

fn main() {
    println!("Start listening on port {}", 3000);
    Iron::new(routes::create_router())
        .http("localhost:3000")
        .unwrap();
}
