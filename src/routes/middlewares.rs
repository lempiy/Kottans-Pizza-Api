use iron::{status, IronError, IronResult, Request, Response};
use iron::headers::{Authorization, Bearer, ContentType};
use iron::{AfterMiddleware, BeforeMiddleware};
use std::sync::{Arc, Mutex};
use utils::types::StringError;
use utils::jwt::check;
use iron::modifiers;
use redis::Connection;

pub struct JsonAfterMiddleware;

impl AfterMiddleware for JsonAfterMiddleware {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(ContentType::json());
        Ok(res)
    }
}

pub struct AuthBeforeMiddleware {
    rds: Arc<Mutex<Connection>>,
}

impl AuthBeforeMiddleware {
    pub fn new(rds: Arc<Mutex<Connection>>) -> AuthBeforeMiddleware {
        AuthBeforeMiddleware { rds }
    }
}

impl BeforeMiddleware for AuthBeforeMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let response = r#"{"success": false, "error": "Wrong authorization data"}"#;
        let rds = self.rds.lock().unwrap();
        let result = match req.headers
            .get::<Authorization<Bearer>>()
            .ok_or(StringError("No auth header".to_string()))
        {
            Ok(bearer) => match check(&rds, bearer.token.to_owned()) {
                Ok(data) => Ok(data.claims),
                Err(e) => Err(IronError::new(
                    e,
                    (
                        status::Forbidden,
                        modifiers::Header(ContentType::json()),
                        response,
                    ),
                )),
            },
            Err(e) => Err(IronError::new(
                e,
                (
                    status::Forbidden,
                    modifiers::Header(ContentType::json()),
                    response,
                ),
            )),
        };

        match result {
            Ok(claims) => {
                // TODO: use req.extensions.insert instead
                req.headers
                    .append_raw("x-store-id", claims.store_id.to_string().into_bytes());
                req.headers
                    .append_raw("x-user-uuid", claims.uuid.to_string().into_bytes());
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}

pub struct NotFound404;

impl AfterMiddleware for NotFound404 {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        if let Some(s) = err.response.status {
            if s == status::NotFound {
                Ok(Response::with(
                    (status::NotFound, r#"{"status": "404 Not Found"}"#),
                ))
            } else {
                Err(err)
            }
        } else {
            Err(err)
        }
    }
}
