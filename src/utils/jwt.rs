use jwt::{encode, decode, Header, Algorithm, Validation, TokenData};
use uuid::Uuid;
use chrono::offset::Utc;
use chrono::Duration;
use jwt::errors::{Result, Error, ErrorKind};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    exp: i64,
    username: String,
    uuid: Uuid,
}

pub fn generate(username: &str, uuid: Uuid)->Result<String> {
    let secret = "secret".as_ref();
    let exp = (Utc::now() + Duration::hours(5)).naive_utc().timestamp();
    let claims = Claims {
        exp,
        username: username.to_string(),
        uuid,
    };
    encode(&Header::default(), &claims, secret)
}

pub fn check(token: String) -> Result<TokenData<Claims>> {
    let secret = "secret".as_ref();
    let now = Utc::now().naive_utc().timestamp();
    match decode::<Claims>(&token, secret, &Validation::default()) {
        Ok(token) => {
            if now > token.claims.exp {
                Ok(token)
            } else {
                Err(ErrorKind::ExpiredSignature.into())
            }
        },
        Err(e) => Err(e)
    }
}
