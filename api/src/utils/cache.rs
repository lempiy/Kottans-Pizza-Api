use redis::{Client, Commands, Connection, ErrorKind, PubSub, RedisError, RedisResult};
use std::sync::MutexGuard;
use uuid::Uuid;
use serde_json;

#[derive(Serialize)]
pub struct WsTicket {
    pub token: String,
    pub user_uuid: String,
    pub store_id: i32,
}

pub const WS_TICKET_EXPIRATION_TIME: usize = 60;

pub fn create_redis_connection() -> (Connection, PubSub) {
    let client = Client::open("redis://127.0.0.1:6379").expect("Cannot dial redis");
    (
        client
            .get_connection()
            .expect("Cannot get redis connection"),
        client.get_pubsub().expect("Cannot get redis pubsub"),
    )
}

pub fn set_session(
    rds: &MutexGuard<Connection>,
    uuid: Uuid,
    device_uuid: Uuid,
    secret: Uuid,
    exp: i64,
) -> RedisResult<(String, Uuid)> {
    match rds.set::<String, String, String>(
        uuid.to_string() + &device_uuid.to_string(),
        secret.to_string(),
    ) {
        Ok(_) => if let Err(e) = rds.expire_at::<String, usize>(
            uuid.to_string() + &device_uuid.to_string(),
            exp as usize,
        ) {
            Err(e)
        } else {
            Ok((secret.to_string(), device_uuid))
        },
        Err(e) => Err(e),
    }
}

pub fn get_session(
    rds: &MutexGuard<Connection>,
    uuid: Uuid,
    device_uuid: Uuid,
) -> RedisResult<String> {
    rds.get(uuid.to_string() + &device_uuid.to_string())
}

pub fn set_ws_ticket(
    rds: &MutexGuard<Connection>,
    token: String,
    user_uuid: String,
    store_id: i32,
) -> RedisResult<()> {
    let ticket = WsTicket {
        token,
        user_uuid,
        store_id,
    };
    match serde_json::to_string(&ticket) {
        Ok(s) => match rds.set::<String, String, String>(ticket.token.clone(), s) {
            Ok(_) => if let Err(e) =
                rds.expire::<String, usize>(ticket.token, WS_TICKET_EXPIRATION_TIME)
            {
                Err(e)
            } else {
                Ok(())
            },
            Err(e) => Err(e),
        },
        Err(_) => Err(RedisError::from(
            (ErrorKind::IoError, "Error upon serializing ticket data"),
        )),
    }
}
