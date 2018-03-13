use std::sync::{Arc, Mutex};
use postgres::Connection;
use iron::{status, headers, Handler, IronResult, Request, Response, Plugin};
use serde_json;
use params::{Params, Value, Map};
use models::tag::Tag;
use std::error::Error;

// Get tag list
pub struct GetTagListHandler {
    database: Arc<Mutex<Connection>>,
}

impl GetTagListHandler {
    pub fn new(database: Arc<Mutex<Connection>>) -> GetTagListHandler {
        GetTagListHandler { database }
    }
}

impl Handler for GetTagListHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        req.headers.remove::<headers::ContentType>();
        let map:&Map = try_handler!(req.get_ref::<Params>());
        let offset = match map.find(&["offset"]) {
            Some(&Value::String(ref s)) => {
                s.to_owned().parse::<i64>().ok()
            },
            _ => None,
        };
        let limit = match map.find(&["limit"]) {
            Some(&Value::String(ref s)) => {
                s.to_owned().parse::<i64>().ok()
            },
            _ => None,
        };
        let mg = self.database.lock().unwrap();
        let response = try_handler!(Tag::get_some(&mg, offset, limit));
        let res: String = try_handler!(serde_json::to_string(&response));
        Ok(Response::with((status::Ok, res)))
    }
}