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
use std::env;

fn main() {
    let port = match env::var_os("PORT") {
        Some(val) => val.into_string().unwrap(),
        None => "3000".to_string(),
    };
    println!("Start listening on port 0.0.0.0:{}", port);
    Iron::new(routes::create_router())
        .http(format!("0.0.0.0:{}", port))
        .unwrap();
}
