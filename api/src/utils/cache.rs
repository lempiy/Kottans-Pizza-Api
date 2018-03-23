use redis::{Client, Commands, Connection, PubSub, RedisResult};
use std::sync::MutexGuard;
use uuid::Uuid;

pub fn create_redis_connection() -> (Connection, PubSub) {
    let client = Client::open("redis://127.0.0.1:6379")
        .expect("Cannot dial redis");
    (client.get_connection()
        .expect("Cannot get redis connection"),
     client.get_pubsub()
         .expect("Cannot get redis pubsub"))
}

pub fn set_session(
    rds: &MutexGuard<Connection>,
    uuid: Uuid,
    secret: Uuid,
    exp: i64,
) -> RedisResult<String> {
    match rds.set::<String, String, String>(uuid.to_string(), secret.to_string()) {
        Ok(_) => if let Err(e) = rds.expire_at::<String, usize>(uuid.to_string(), exp as usize) {
            Err(e)
        } else {
            Ok(secret.to_string())
        },
        Err(e) => Err(e),
    }
}

pub fn get_session(rds: &MutexGuard<Connection>, uuid: Uuid) -> RedisResult<String> {
    rds.get(uuid.to_string())
}
