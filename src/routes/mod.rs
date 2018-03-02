use models;
use handlers::*;
use router::Router;
use iron::Handler;
use iron::prelude::Chain;
use env_logger;
use logger::Logger;

mod middlewares;
use iron_cors::CorsMiddleware;
use mount::Mount;

pub fn create_router() -> Chain {
    env_logger::init().unwrap();

    let db = models::create_db_connection();
    let handler = Handlers::new(db);

    let mut users_router = Router::new();

    users_router.post("/create", handler.user_create, "create_user");
    users_router.post("/login", handler.user_login, "login");
    users_router.get("/my_info", auth_only(handler.user_info), "my_info");


    let mut mount = Mount::new();
    mount
        .mount("/api/v1/user", users_router);

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

fn auth_only<H: Handler>(handler: H) -> Chain {
    let auth_only_middleware = middlewares::AuthBeforeMiddleware;
    let mut chain = Chain::new(handler);
    chain.link_before(auth_only_middleware);
    chain
}
