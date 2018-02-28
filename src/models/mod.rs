use postgres::{Connection, TlsMode};

pub mod user;

pub fn create_db_connection() -> Connection {
    Connection::connect("postgres://db_user:xxpassxx@localhost:5432/pizza",
                        TlsMode::None).unwrap()
}

