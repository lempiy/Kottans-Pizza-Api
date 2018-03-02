use redis::{Client, Commands, Connection, RedisResult};
use std::sync::MutexGuard;
use uuid::Uuid;

pub fn create_redis_connection() -> Connection {
    Client::open("redis://127.0.0.1:6379")
        .expect("Cannot dial redis")
        .get_connection()
        .expect("Cannot get redis connection")
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
