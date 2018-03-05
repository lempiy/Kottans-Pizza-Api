use std::sync::{Arc, Mutex};
use postgres::Connection;
use iron::{status, headers, Handler, IronResult, Request, Response, Plugin};
use serde_json;
use params::{Params, Value, Map};
use models::ingredient::Ingredient;
use std::error::Error;

// Get ingredient list
pub struct GetIngredientListHandler {
    database: Arc<Mutex<Connection>>,
}

impl GetIngredientListHandler {
    pub fn new(database: Arc<Mutex<Connection>>) -> GetIngredientListHandler {
        GetIngredientListHandler { database }
    }
}

impl Handler for GetIngredientListHandler {
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
        let response = try_handler!(Ingredient::get_some(&mg, offset, limit));
        let res: String = try_handler!(serde_json::to_string(&response));
        Ok(Response::with((status::Ok, res)))
    }
}