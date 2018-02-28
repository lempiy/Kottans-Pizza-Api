use models;
use handlers::*;
use router::Router;
use iron::prelude::Chain;
use env_logger;
use logger::Logger;

mod middlewares;

pub fn create_router() -> Chain {
    let mut router = Router::new();
    env_logger::init().unwrap();
    let (logger_before, logger_after) = Logger::new(None);

    let db = models::create_db_connection();
    let handler = Handlers::new(db);

    router.post("/create_user", handler.user_create, "create_user");

    let json_content_middleware = middlewares::JsonAfterMiddleware;

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(json_content_middleware);
    chain.link_after(logger_after);
    chain
}

