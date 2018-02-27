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

mod models;
mod handlers;

fn main() {
    let db = models::create_db_connection();
    let handler = handlers::Handlers::new(db);
}
