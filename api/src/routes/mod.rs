use models;
use utils::cache;
use handlers::*;
use router::Router;
use iron::Handler;
use iron::prelude::Chain;
use env_logger;
use logger::Logger;
use multipart::server::iron::Intercept;

mod middlewares;
use iron_cors::CorsMiddleware;
use mount::Mount;
use std::sync::{Arc, Mutex};
use redis::Connection;
use utils::s3_uploader;
use utils::pubsub::Manager;

pub fn create_router() -> Chain {
    env_logger::init().unwrap();
    let db = models::create_db_connection();
    let (connection, pubsub) = cache::create_redis_connection();
    let redis = Arc::new(Mutex::new(connection));
    let psm = Manager::new(pubsub, redis.clone());
    let ps_manager = Arc::new(Mutex::new(psm));
    let s3_client = Arc::new(Mutex::new(s3_uploader::configure_s3_client()));
    let handler = Handlers::new(db, redis.clone(), s3_client.clone(), ps_manager.clone());

    let mut users_router = Router::new();

    users_router.post("/create", handler.user_create, "create_user");
    users_router.post("/login", handler.user_login, "login");
    users_router.get(
        "/my_info",
        auth_only(handler.user_info, redis.clone()),
        "my_info",
    );

    let mut ingredient_router = Router::new();
    ingredient_router.get(
        "/list",
        auth_only(handler.ingredient_list, redis.clone()),
        "ingredient_list",
    );

    let mut tag_router = Router::new();
    tag_router.get(
        "/list",
        auth_only(handler.tag_list, redis.clone()),
        "tag_list",
    );

    let mut store_router = Router::new();
    store_router.get("/list", handler.store_list, "store_list");

    let mut pizza_router = Router::new();
    let mut chain = Chain::new(handler.pizza_create);
    let interceptor = Intercept::default().file_size_limit(5 << 20);
    chain.link_before(interceptor);
    pizza_router.post("/create", auth_only(chain, redis.clone()), "pizza_create");
    pizza_router.get(
        "/list",
        auth_only(handler.pizza_list, redis.clone()),
        "pizza_list",
    );

    let mut ws_router = Router::new();
    ws_router.get(
        "/ticket",
        auth_only(handler.ws_ticket, redis.clone()),
        "ticket",
    );

    let mut index_router = Router::new();
    index_router.get("/", handler.index_handler, "index");

    let mut mount = Mount::new();
    mount.mount("/api/v1/user", users_router);
    mount.mount("/api/v1/ingredient", ingredient_router);
    mount.mount("/api/v1/tag", tag_router);
    mount.mount("/api/v1/store", store_router);
    mount.mount("/api/v1/pizza", pizza_router);
    mount.mount("/api/v1/ws", ws_router);
    mount.mount("/", index_router);

    apply_middlewares(mount)
}

fn apply_middlewares(mount: Mount) -> Chain {
    let (logger_before, logger_after) = Logger::new(None);
    let json_content_middleware = middlewares::JsonAfterMiddleware;
    let cors_middleware = CorsMiddleware::with_allow_any();
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
