#[macro_use]
mod macros;
mod user;
mod ingredient;
mod store;
mod tag;
mod pizza;

use std::sync::{Arc, Mutex};
use postgres::Connection;
use redis;
use iron::{status, Handler, IronResult, Request, Response};
use rusoto_s3::S3Client;

pub struct Handlers {
    pub user_create: user::UserCreateHandler,
    pub user_login: user::UserLoginHandler,
    pub user_info: user::UserInfoHandler,

    pub ingredient_list: ingredient::GetIngredientListHandler,
    pub tag_list: tag::GetTagListHandler,

    pub store_list: store::GetStoreListHandler,

    pub pizza_create: pizza::CreatePizzaHandler,
    pub pizza_list: pizza::GetPizzaListHandler,

    pub index_handler: IndexHandler,
}

impl Handlers {
    pub fn new(
        db: Connection,
        rds: Arc<Mutex<redis::Connection>>,
        s3_client: Arc<Mutex<S3Client>>,
    ) -> Handlers {
        let database = Arc::new(Mutex::new(db));
        Handlers {
            user_create: user::UserCreateHandler::new(database.clone()),
            user_login: user::UserLoginHandler::new(database.clone(), rds.clone()),
            user_info: user::UserInfoHandler::new(database.clone()),

            ingredient_list: ingredient::GetIngredientListHandler::new(database.clone()),

            tag_list: tag::GetTagListHandler::new(database.clone()),

            store_list: store::GetStoreListHandler::new(database.clone()),

            pizza_create: pizza::CreatePizzaHandler::new(database.clone(), s3_client.clone()),
            pizza_list: pizza::GetPizzaListHandler::new(database.clone()),

            index_handler: IndexHandler::new(),
        }
    }
}

pub struct IndexHandler;

impl IndexHandler {
    pub fn new() -> IndexHandler {
        IndexHandler {}
    }
}

impl Handler for IndexHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok,
        r#"{"name": "Kottans Pizza Api","source": "https://github.com/lempiy/Kottans-Pizza-Api"}"#)
        ))
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
