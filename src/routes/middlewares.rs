use iron::{status,Response, IronResult, Request, IronError};
use iron::headers::{ContentType, Authorization, Bearer};
use iron::{AfterMiddleware, BeforeMiddleware};
use iron::AroundMiddleware;
use utils::types::{StringError};
use utils::jwt::check;
use iron::modifiers;

pub struct JsonAfterMiddleware;

impl AfterMiddleware for JsonAfterMiddleware {
    fn after(&self, _:&mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(ContentType::json());
        Ok(res)
    }
}

pub struct AuthBeforeMiddleware;

impl BeforeMiddleware for AuthBeforeMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let response = r#"{"success": false, "error": "Wrong authorization data"}"#;
        match req.headers.get::<Authorization<Bearer>>()
            .ok_or(StringError("No auth header".to_string())) {
            Ok(bearer) => {
                match check(bearer.token.to_owned()) {
                    Ok(_) => {
                        Ok(())
                    }
                    Err(e) => {
                        Err(IronError::new(e, (status::Forbidden,
                           modifiers::Header(ContentType::json()), response)))
                    }
                }
            }
            Err(e)=> {
                Err(IronError::new(e, (status::Forbidden,
                   modifiers::Header(ContentType::json()), response)))
            }
        }
    }
}
