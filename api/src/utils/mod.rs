pub mod jwt;
pub mod types;
pub mod cache;
pub mod s3_uploader;
pub mod validator;
pub mod calculator;
pub mod pubsub;
pub mod constants;

pub fn itob(n: i32) -> bool {
    n != 0
}
