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
use std::fs::File;

struct CreatePizzaData{
    image: File,
    name: String,
    size: i64,
    description: Option<String>,
    tags: Vec<i32>,
    ingredients: Vec<i32>
}

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
        let create_pizza_data = match extract_pizza_data(map) {
            Some(data) => data,
            _ => {
                let response = super::ErrorResponse {
                    success: false,
                    error: "Required field(s) in form data are missing".to_string(),
                };
                let res: String = try_handler!(serde_json::to_string(&response));
                return Ok(Response::with((status::BadRequest, res)))
            }
        };

        Ok(Response::with((status::Ok)))
    }
}

fn extract_pizza_data(map: &Map)-> Option<CreatePizzaData> {
    Some(CreatePizzaData{
        image: match map.find(&["image"]) {
            Some(&Value::File(ref file)) => {
                match file.open() {
                    Ok(f) => f,
                    _ => {
                        return None
                    }
                }
            }
            _ => return None
        },
        name: match map.find(&["name"]) {
            Some(&Value::String(ref s)) => s.to_string(),
            _ => return None
        },
        description: match map.find(&["description"]) {
            Some(&Value::String(ref s)) => Some(s.to_string()),
            _ => None
        },
        size: match map.find(&["size"]) {
            Some(&Value::I64(ref n)) => *n,
            _ => return None
        },
        tags: match map.find(&["tags"]) {
            Some(&Value::String(ref s)) => {
                match serde_json::from_str::<Vec<i32>>(s) {
                    Ok(ids) => ids,
                    _ => return None
                }
            },
            _ => return None
        },
        ingredients: match map.find(&["ingredients"]) {
            Some(&Value::String(ref s)) => {
                match serde_json::from_str::<Vec<i32>>(s) {
                    Ok(ids) => ids,
                    _ => return None
                }
            },
            _ => return None
        },
    })
}
