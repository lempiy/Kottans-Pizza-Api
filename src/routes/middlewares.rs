use iron::{Response, IronResult, Request};
use iron::headers::ContentType;
use iron::{AfterMiddleware, BeforeMiddleware};

pub struct JsonAfterMiddleware;

impl AfterMiddleware for JsonAfterMiddleware {
    fn after(&self, _:&mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(ContentType::json());
        Ok(res)
    }
}
