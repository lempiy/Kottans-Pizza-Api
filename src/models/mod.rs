use postgres::{Connection, TlsMode};
use std::sync::{Arc, Mutex};

pub mod user;

//TODO: get from config
pub const DATABASE:Arc<Mutex<Connection>> = Arc::new(Mutex::new(
    Connection::connect("postgres://postgres@localhost:5433", TlsMode::None)
        .unwrap()));
