use std::sync::{Arc, Mutex};
use postgres::Connection;
use iron::headers::ContentType;
use iron::mime::Mime;
use iron::mime::TopLevel::Multipart;
use iron::mime::SubLevel::FormData;
use iron::{status, headers, Handler, IronResult, Request, Response, Plugin};
use serde_json;
use params::{Params, Value, Map};
use models::pizza::Pizza;
use std::error::Error;
use utils::types::StringError;

// Create new pizza
pub struct CreatePizzaHandler {
    database: Arc<Mutex<Connection>>,
}

impl CreatePizzaHandler {
    pub fn new(database: Arc<Mutex<Connection>>) -> CreatePizzaHandler {
        CreatePizzaHandler { database }
    }
}

impl Handler for CreatePizzaHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match req.headers.get() {
            Some(&ContentType(Mime(Multipart, FormData, _))) => (),
            _ => {
                let response = super::ErrorResponse {
                    success: false,
                    error: "Wrong Content-Type".to_string(),
                };
                let res: String = try_handler!(serde_json::to_string(&response));
                return Ok(Response::with((status::BadRequest, res)))
            }
        };
        let map:&Map = try_handler!(req.get_ref::<Params>());
        match map.find(&["image"]) {
            Some(&Value::File(ref file)) => {
                println!("{:?}", file.path)
            }
            _ => {
                println!("no file");
            }
        };
        Ok(Response::with((status::Ok)))
    }
}
