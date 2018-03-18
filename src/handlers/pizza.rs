use std::sync::{Arc, Mutex, MutexGuard};
use postgres::Connection;
use iron::headers::ContentType;
use iron::mime::Mime;
use iron::mime::TopLevel::Multipart;
use iron::mime::SubLevel::FormData;
use iron::{status, headers, Handler, IronResult, Request, Response, Plugin};
use serde_json;
use params::{Params, Value, Map};
use models::pizza::{Pizza, CreatePizzaInput};
use std::error::Error;
use utils::types::StringError;
use utils::s3_uploader::put_object_with_filename;
use std::fs::File;
use rusoto_s3::{S3Client, PutObjectOutput};
use std::str::FromStr;
use uuid;

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
    s3_client: Arc<Mutex<S3Client>>,
}

impl CreatePizzaHandler {
    pub fn new(database: Arc<Mutex<Connection>>, s3_client: Arc<Mutex<S3Client>>) -> CreatePizzaHandler {
        CreatePizzaHandler { database, s3_client }
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
        let store_id = try_store_id!(req.headers);
        let user_uuid = try_handler!(
            uuid::Uuid::from_str(try_user_uuid!(req.headers).as_ref())
        );
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
        let s3_client = self.s3_client.lock().unwrap();
        let db = self.database.lock().unwrap();
        let uid = uuid::Uuid::new_v4();
        let name = format!("{}_pizza.png", uid);
        let result:PutObjectOutput =
            try_handler!(put_object_with_filename(&s3_client,
                                     "pizza-kottans",
                                     create_pizza_data.image,
            name.as_ref()));
        let input = CreatePizzaInput{
            uuid: uid,
            name: create_pizza_data.name,
            store_id,
            user_uuid,
            price: 0.0,
            size: create_pizza_data.size as i32,
            description: create_pizza_data.description,
            tags: create_pizza_data.tags,
            img_url: name,
            ingredients: create_pizza_data.ingredients,
            preparation_sec: 60*3,
        };
        try_handler!(Pizza::create(&db, input));
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
