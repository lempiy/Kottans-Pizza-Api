use models;
use utils::cache;
use handlers::*;
use router::Router;
use iron::Handler;
use iron::prelude::Chain;
use env_logger;
use logger::Logger;

mod middlewares;
use iron_cors::CorsMiddleware;
use mount::Mount;
use std::sync::{Arc, Mutex};
use redis::Connection;

pub fn create_router() -> Chain {
    env_logger::init().unwrap();

    let db = models::create_db_connection();
    let redis = Arc::new(Mutex::new(cache::create_redis_connection()));

    let handler = Handlers::new(db, redis.clone());

    let mut users_router = Router::new();

    users_router.post("/create", handler.user_create, "create_user");
    users_router.post("/login", handler.user_login, "login");
    users_router.get("/my_info", auth_only(handler.user_info, redis), "my_info");

    let mut index_router = Router::new();
    index_router.get("/", handler.index_handler, "index");

    let mut mount = Mount::new();
    mount.mount("/api/v1/user", users_router);
    mount.mount("/", index_router);

    apply_middlewares(mount)
}

fn apply_middlewares(mount: Mount) -> Chain {
    let (logger_before, logger_after) = Logger::new(None);
    let json_content_middleware = middlewares::JsonAfterMiddleware;
    let cors_middleware = CorsMiddleware::with_allow_any(true);
    let not_found_middleware = middlewares::NotFound404;

    let mut chain = Chain::new(mount);
    chain
        .link_before(logger_before)
        .link_around(cors_middleware)
        .link_after(not_found_middleware)
        .link_after(json_content_middleware)
        .link_after(logger_after);
    chain
}

fn auth_only<H: Handler>(handler: H, rds: Arc<Mutex<Connection>>) -> Chain {
    let auth_only_middleware = middlewares::AuthBeforeMiddleware::new(rds);
    let mut chain = Chain::new(handler);
    chain.link_before(auth_only_middleware);
    chain
}
