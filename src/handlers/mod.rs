#[macro_use]
mod macros;
mod user;

use std::sync::{Arc, Mutex};
use postgres::Connection;

pub struct Handlers {
    pub user_create: user::UserCreateHandler,
    pub user_login: user::UserLoginHandler,
    pub user_info: user::UserInfoHandler,
}

impl Handlers {
    pub fn new(db: Connection) -> Handlers {
        let database = Arc::new(Mutex::new(db));
        Handlers {
            user_create: user::UserCreateHandler::new(database.clone()),
            user_login: user::UserLoginHandler::new(database.clone()),
            user_info: user::UserInfoHandler::new(database.clone()),
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    error: String,
}

#[derive(Serialize)]
struct ErrorResponseWithValidation {
    success: bool,
    error: String,
    validations: Vec<String>,
}
