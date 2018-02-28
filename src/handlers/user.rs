use std::sync::{Arc, Mutex};
use postgres::Connection;
use iron::{status, AfterMiddleware, Handler, Request, IronResult, Response};
use std::io::Read;
use models::user::User;
use uuid::Uuid;
use serde_json;
use std::error::Error;

use validator::{Validate, ValidationError, ValidationErrors};

// Create user

pub struct UserCreateHandler {
    database: Arc<Mutex<Connection>>,
}

impl UserCreateHandler {
    pub fn new(database: Arc<Mutex<Connection>>) -> UserCreateHandler {
        UserCreateHandler{database}
    }
}

#[derive(Validate, Deserialize)]
struct CreateUserRequest {
    #[validate(length(min="2",max="24",message="Username is not valid. Min length is 2, max - is 24"))]
    username: String,
    #[validate(email(message="Email is not valid"))]
    email: String,
    #[validate(length(min="8",message="Password is not valid. Min length is 8"))]
    password: String,
    #[validate(must_match(other="password",message="Passwords do not match"))]
    password_repeat: String,
}

#[derive(Serialize)]
struct CreateUserResponse {
    success: bool,
    uuid: Uuid,
}

impl Handler for UserCreateHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        try_handler!(req.body.read_to_string(&mut payload));
        let mut user_data: CreateUserRequest =
            try_handler!(serde_json::from_str(payload.as_ref()), status::BadRequest);
        let mg = self.database.lock().unwrap();

        try_validate!(user_data.validate(),
            vec![User::validate_unique_username(&mg, user_data.username.as_ref())]);
        let user: User = try_handler!(User::new(
            &mg,
            user_data.username.as_ref(),
            user_data.email.as_ref(),
            user_data.password.as_ref(),
        ));
        let response = CreateUserResponse{
            success: true,
            uuid: user.uuid.clone()
        };
        let res: String = try_handler!(serde_json::to_string(&response));
        Ok(Response::with((status::Created, res)))
    }
}
