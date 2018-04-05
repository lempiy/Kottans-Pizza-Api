use std::sync::{Arc, Mutex};
use postgres::Connection;
use iron::headers::{Authorization, Bearer};
use iron::{status, Handler, IronResult, Request, Response};
use std::io::Read;
use models::user::User;
use models::store::Store;
use uuid::Uuid;
use serde_json;
use std::error::Error;
use utils::types::StringError;
use utils::jwt::{self, get_claims, Claims};
use chrono::{DateTime, Duration, Utc};
use redis;
use utils::cache::{set_session, set_ws_ticket, WS_TICKET_EXPIRATION_TIME};
use utils::random_token;

use validator::{Validate, ValidationError};

// Create user

pub struct UserCreateHandler {
    database: Arc<Mutex<Connection>>,
}

impl UserCreateHandler {
    pub fn new(database: Arc<Mutex<Connection>>) -> UserCreateHandler {
        UserCreateHandler { database }
    }
}

#[derive(Validate, Deserialize)]
struct CreateUserRequest {
    #[validate(length(min = "2", max = "24",
                      message = "Username is not valid. Min length is 2, max - is 24"))]
    username: String,
    #[validate(email(message = "Email is not valid"))] email: String,
    #[validate(length(min = "8", message = "Password is not valid. Min length is 8"))]
    password: String,
    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    password_repeat: String,
    store_id: i32,
    #[validate(length(min = "8", message = "Store password is not valid. Min length is 8"))]
    store_password: String,
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
            try_handler!(serde_json::from_str(payload.as_ref()), status::BadRequest);
        let mg = self.database.lock().unwrap();
        try_validate!(
            user_data.validate(),
            vec![
                Store::validate_correct_store(
                    &mg,
                    user_data.store_id,
                    user_data.store_password.as_ref(),
                ),
                User::validate_unique_username(&mg, user_data.username.as_ref()),
            ]
        );

        let user: User = try_handler!(User::new(
            &mg,
            user_data.store_id,
            user_data.username.as_ref(),
            user_data.email.as_ref(),
            user_data.password.as_ref(),
        ));
        let response = CreateUserResponse {
            success: true,
            uuid: user.uuid.clone(),
        };
        let res: String = try_handler!(serde_json::to_string(&response));
        Ok(Response::with((status::Created, res)))
    }
}

// Login

pub struct UserLoginHandler {
    database: Arc<Mutex<Connection>>,
    rds: Arc<Mutex<redis::Connection>>,
}

impl UserLoginHandler {
    pub fn new(
        database: Arc<Mutex<Connection>>,
        rds: Arc<Mutex<redis::Connection>>,
    ) -> UserLoginHandler {
        UserLoginHandler { database, rds }
    }
}

#[derive(Validate, Deserialize)]
struct UserLoginRequest {
    #[validate(length(min = "2", max = "24",
                      message = "Username is not valid. Min length is 2, max - is 24"))]
    username: String,
    #[validate(length(min = "8", message = "Password is not valid. Min length is 8"))]
    password: String,
}

#[derive(Serialize)]
struct UserLoginResponse {
    success: bool,
    token: String,
}

impl Handler for UserLoginHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        try_handler!(req.body.read_to_string(&mut payload));
        let user_data: UserLoginRequest =
            try_handler!(serde_json::from_str(payload.as_ref()), status::BadRequest);
        try_validate!(user_data.validate());
        let mg = self.database.lock().unwrap();
        let rds = self.rds.lock().unwrap();

        let result: Option<User> = try_handler!(User::find(
            &mg,
            user_data.username.as_ref(),
            user_data.password.as_ref()
        ));

        if let Some(user) = result {
            try_handler!(User::update_login(&mg, user.uuid));
            let exp = (Utc::now() + Duration::hours(5)).naive_utc().timestamp();
            let (secret, device_uuid) = try_handler!(set_session(
                &rds,
                user.uuid,
                Uuid::new_v4(),
                Uuid::new_v4(),
                exp
            ));
            let token = try_handler!(jwt::generate(
                user.username.as_ref(),
                user.uuid,
                secret,
                device_uuid,
                exp,
                user.store_id
            ));
            let response = UserLoginResponse {
                success: true,
                token,
            };
            let res: String = try_handler!(serde_json::to_string(&response));
            Ok(Response::with((status::Ok, res)))
        } else {
            let response = super::ErrorResponse {
                success: false,
                error: "Wrong username or password".to_string(),
            };
            let res: String = try_handler!(serde_json::to_string(&response));
            Ok(Response::with((status::BadRequest, res)))
        }
    }
}

// Get ws token

#[derive(Serialize)]
struct GetWsTokenResponse {
    success: bool,
    info: String,
    token: String,
}

pub struct UserGetWsTokenHandler {
    rds: Arc<Mutex<redis::Connection>>,
}

impl UserGetWsTokenHandler {
    pub fn new(rds: Arc<Mutex<redis::Connection>>) -> UserGetWsTokenHandler {
        UserGetWsTokenHandler { rds }
    }
}

impl Handler for UserGetWsTokenHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let store_id = try_store_id!(req.headers);
        let user_uuid = try_user_uuid!(req.headers);

        let token = random_token();
        let rds = self.rds.lock().unwrap();
        try_handler!(set_ws_ticket(&rds, token.clone(), user_uuid, store_id));
        let response = GetWsTokenResponse {
            success: true,
            info: format!(
                "Your ws connection token is valid for {}s",
                WS_TICKET_EXPIRATION_TIME
            ),
            token,
        };
        let res: String = try_handler!(serde_json::to_string(&response));
        Ok(Response::with((status::BadRequest, res)))
    }
}

// My info

pub struct UserInfoHandler {
    database: Arc<Mutex<Connection>>,
}

impl UserInfoHandler {
    pub fn new(database: Arc<Mutex<Connection>>) -> UserInfoHandler {
        UserInfoHandler { database }
    }
}

#[derive(Serialize)]
struct UserInfoResponse {
    username: String,
    uuid: Uuid,
    email: String,
    created_at: DateTime<Utc>,
    last_login: Option<DateTime<Utc>>,
}

impl Handler for UserInfoHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let bearer: &Authorization<Bearer> = try_handler!(
            req.headers
                .get::<Authorization<Bearer>>()
                .ok_or(StringError("Server error".to_string()))
        );
        let claims: Claims = try_handler!(get_claims(bearer.token.to_owned()));
        let mg = self.database.lock().unwrap();
        let store_id = try_store_id!(req.headers);
        let result: Option<User> = try_handler!(User::get(&mg, claims.uuid, store_id));
        match result {
            Some(user) => {
                let response = UserInfoResponse {
                    username: user.username,
                    uuid: user.uuid,
                    email: user.email,
                    created_at: user.created_at,
                    last_login: user.last_login,
                };
                let res: String = try_handler!(serde_json::to_string(&response));
                Ok(Response::with((status::Ok, res)))
            }
            None => {
                let response = super::ErrorResponse {
                    success: false,
                    error: "User not found".to_string(),
                };
                let res: String = try_handler!(serde_json::to_string(&response));
                Ok(Response::with((status::NotFound, res)))
            }
        }
    }
}
