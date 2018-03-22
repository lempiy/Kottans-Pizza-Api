use std::sync::{Arc, Mutex};
use postgres::Connection;
use iron::headers::ContentType;
use iron::mime::Mime;
use iron::mime::TopLevel::Multipart;
use iron::mime::SubLevel::FormData;
use iron::{status, Handler, IronResult, Request, Response, Plugin, headers};
use serde_json;
use models::pizza::{Pizza, CreatePizzaInput};
use std::error::Error;
use utils::s3_uploader::put_object_with_filename;
use utils::validator::{ValidationFile, validate_image, validate_pizza_size};
use params::{Params, Value, Map};
use rusoto_s3::{S3Client};
use std::str::FromStr;
use chrono::DateTime;
use chrono::offset::Utc;
use uuid;
use validator::{Validate,ValidationError};
use utils::calculator::{calculate_pizza_price, calculate_preparation_time};
use models::ingredient::Ingredient;
use models::tag::Tag;
use std::fs::File;
use multipart::server::Entries;
use utils::types::StringError;

#[derive(Validate)]
struct CreatePizzaData{
    #[validate(custom="validate_image")]
    image: ValidationFile,
    #[validate(length(min = "3", max = "24",
    message = "Pizza name is not valid. Min length is 3, max - is 24"))]
    name: String,
    #[validate(custom(function = "validate_pizza_size",
    message = "Pizza size can be either 30, 45 or 60"))]
    size: i64,
    description: Option<String>,
    tags: Vec<i32>,
    ingredients: Vec<i32>
}

#[derive(Serialize)]
struct CreateResponse {
    success: bool,
    time_prepared: DateTime<Utc>,
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
        let entries = try_handler!(req.extensions.get_mut::<Entries>()
            .ok_or(StringError("Cannot extract multipart form fields".to_string())));
        let create_pizza_data = match process_entries(entries) {
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
        let db = self.database.lock().unwrap();
        try_validate!(
            create_pizza_data.validate(),
            vec![
                Ingredient::validate_ingredients_exist(&db, &create_pizza_data.ingredients),
                Tag::validate_tags_exist(&db, &create_pizza_data.tags),
            ]
        );
        let s3_client = self.s3_client.lock().unwrap();
        let uid = uuid::Uuid::new_v4();
        let name = format!("{}_pizza.png", uid);
        let file = File::open(create_pizza_data.image.file.path.clone()).unwrap();
        try_handler!(put_object_with_filename(&s3_client,
                                 "pizza-kottans",
                                 file,
        name.as_ref()));
        let time_prepared = calculate_preparation_time(
            &create_pizza_data.size,
            create_pizza_data.ingredients.len()
        );
        let input = CreatePizzaInput{
            uuid: uid,
            name: create_pizza_data.name,
            store_id,
            user_uuid,
            price:
                try_handler!(
                    calculate_pizza_price(
                        &db,
                        &create_pizza_data.ingredients,
                        &create_pizza_data.size)
                ),
            size: create_pizza_data.size as i32,
            description: create_pizza_data.description,
            tags: create_pizza_data.tags,
            img_url: format!("static/upload/{}", name),
            time_prepared: time_prepared.clone(),
            ingredients: create_pizza_data.ingredients,
        };
        try_handler!(Pizza::create(&db, input));
        let response = CreateResponse {
            success: true,
            time_prepared,
        };
        let res: String = try_handler!(serde_json::to_string(&response));
        Ok(Response::with((status::Ok, res)))
    }
}

fn process_entries(entries: &mut Entries)-> Option<CreatePizzaData> {
    println!("ENTRIES {:?}", entries);
    Some(CreatePizzaData{
        image: match entries.files.get_mut("image") {
            Some(files) => {
                if files.len() == 0 {
                    return None
                };
                ValidationFile{file: files.remove(0)}
            }
            _ => return None
        },
        name:  match entries.fields.get("name") {
            Some(field) => {
                field.to_owned()
            }
            _ => return None
        },
        description:  match entries.fields.get("description") {
            Some(field) => {
                Some(field.to_owned())
            }
            _ => None
        },
        size:  match entries.fields.get("size") {
            Some(field) => {
                match field.to_owned().parse::<i64>() {
                    Ok(n) => n,
                    _ => return None,
                }
            }
            _ => return None
        },
        tags:  match entries.fields.get("tags") {
            Some(field) => {
                match serde_json::from_str::<Vec<i32>>(field) {
                    Ok(ids) => ids,
                    _ => return None
                }
            }
            _ => return None
        },
        ingredients:  match entries.fields.get("ingredients") {
            Some(field) => {
                match serde_json::from_str::<Vec<i32>>(field) {
                    Ok(ids) => ids,
                    _ => return None
                }
            }
            _ => return None
        },
    })
}

// Get pizza list
pub struct GetPizzaListHandler {
    database: Arc<Mutex<Connection>>,
}

impl GetPizzaListHandler {
    pub fn new(database: Arc<Mutex<Connection>>) -> GetPizzaListHandler {
        GetPizzaListHandler { database }
    }
}

impl Handler for GetPizzaListHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        req.headers.remove::<headers::ContentType>();
        let store_id = try_store_id!(req.headers);
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
        let response = try_handler!(Pizza::get_non_accepted(&mg, offset, limit, store_id));
        let res: String = try_handler!(serde_json::to_string(&response));
        Ok(Response::with((status::Ok, res)))
    }
}
