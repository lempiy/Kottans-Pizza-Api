use postgres::{Connection, TlsMode};

pub mod user;

pub fn create_db_connection() -> Connection {
    Connection::connect("postgres://postgres@localhost:5433",
                        TlsMode::None).unwrap()
}

