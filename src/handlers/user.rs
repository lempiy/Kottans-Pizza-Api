use std::sync::{Arc, Mutex};
use postgres::Connection;
use iron::{status, Request, IronResult, Response, Handler};
use std::io::Read;
use models::user::User;
use uuid::Uuid;
use serde_json;
use std::error::Error;

pub struct UserCreateHandler {
    database: Arc<Mutex<Connection>>,
}

impl UserCreateHandler {
    pub fn new(database: Arc<Mutex<Connection>>) -> UserCreateHandler {
        UserCreateHandler{database}
    }
}

#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
    email: String,
    password: String,
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
        let user_data: CreateUserRequest =
            try_handler!(serde_json::from_str(payload.as_ref()));

        let mg = self.database.lock().unwrap();
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
