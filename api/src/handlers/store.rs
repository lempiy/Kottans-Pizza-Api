use std::sync::{Arc, Mutex};
use postgres::Connection;
use iron::{status, Handler, IronResult, Request, Response};
use serde_json;
use models::store::Store;
use std::error::Error;

// Get ingredient list
pub struct GetStoreListHandler {
    database: Arc<Mutex<Connection>>,
}

impl GetStoreListHandler {
    pub fn new(database: Arc<Mutex<Connection>>) -> GetStoreListHandler {
        GetStoreListHandler { database }
    }
}

impl Handler for GetStoreListHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let mg = self.database.lock().unwrap();
        let response = try_handler!(Store::get_all(&mg));
        let res: String = try_handler!(serde_json::to_string(&response));
        Ok(Response::with((status::Ok, res)))
    }
}
