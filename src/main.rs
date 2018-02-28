extern crate chrono;
extern crate env_logger;
extern crate iron;
extern crate logger;
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate uuid;
extern crate postgres;
extern crate jsonwebtoken;
extern crate serde_json;
#[macro_use]
extern crate validator_derive;
extern crate validator;

mod models;
mod handlers;
mod routes;

use iron::Iron;

fn main() {
    println!("Start listening on port {}", 3000);
    Iron::new(routes::create_router()).http("localhost:3000").unwrap();
}
