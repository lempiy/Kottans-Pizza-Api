use models;
use handlers::*;
use router::Router;
use iron::Handler;
use iron::prelude::Chain;
use env_logger;
use logger::Logger;

mod middlewares;
use iron_cors::CorsMiddleware;

pub fn create_router() -> Chain {
    let mut router = Router::new();
    env_logger::init().unwrap();
    let (logger_before, logger_after) = Logger::new(None);

    let db = models::create_db_connection();
    let handler = Handlers::new(db);

    router.post("/create_user", handler.user_create, "create_user");
    router.post("/login", handler.user_login, "login");
    router.get("/my_info", auth_only(handler.user_info), "my_info");

    let json_content_middleware = middlewares::JsonAfterMiddleware;
    let cors_middleware = CorsMiddleware::with_allow_any(true);

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_around(cors_middleware);
    chain.link_after(json_content_middleware);
    chain.link_after(logger_after);
    chain
}

fn auth_only<H: Handler>(handler: H) -> Chain {
    let auth_only_middleware = middlewares::AuthBeforeMiddleware;
    let mut chain = Chain::new(handler);
    chain.link_before(auth_only_middleware);
    chain
}
