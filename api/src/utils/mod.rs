pub mod jwt;
pub mod types;
pub mod cache;
pub mod s3_uploader;
pub mod validator;
pub mod calculator;
pub mod pubsub;
pub mod constants;
use rand::{OsRng, Rng};

pub fn itob(n: i32) -> bool {
    n != 0
}
pub fn random_token() -> String {
    let mut rand_gen = OsRng::new().unwrap();
    rand_gen.gen_ascii_chars().take(32).collect()
}
