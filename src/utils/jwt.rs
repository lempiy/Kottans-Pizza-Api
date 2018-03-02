use jwt::{encode, verify, Algorithm, Header, TokenData};
use uuid::Uuid;
use chrono::offset::Utc;
use jwt::errors::{ErrorKind, Result};
use serde_json;
use base64;
use std::sync::MutexGuard;
use redis::Connection;
use utils::cache::get_session;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: i64,
    pub username: String,
    pub uuid: Uuid,
}

pub fn generate(username: &str, uuid: Uuid, secret: String, exp: i64) -> Result<String> {
    let full_secret = format!("{}_{}", secret, exp);
    let claims = Claims {
        exp,
        username: username.to_string(),
        uuid,
    };

    encode(&Header::default(), &claims, full_secret.as_ref())
}

pub fn check(rds: &MutexGuard<Connection>, token: String) -> Result<TokenData<Claims>> {
    let b64: Vec<_> = token.split(".").collect();
    if b64.len() != 3 {
        return Err(ErrorKind::InvalidToken.into());
    }

    let claims: Claims = match decode_payload(b64[1]) {
        Ok(claims) => claims,
        Err(e) => return Err(e),
    };

    if claims.exp < Utc::now().naive_utc().timestamp() {
        return Err(ErrorKind::ExpiredSignature.into());
    };

    let secret = match get_session(rds, claims.uuid) {
        Ok(secret) => secret,
        Err(_) => return Err(ErrorKind::InvalidSignature.into()),
    };

    if let Ok(valid) = verify_signature(
        b64[2].to_string(),
        (b64[0].to_owned() + "." + b64[1]),
        claims.exp,
        secret,
    ) {
        if valid {
            Ok(TokenData {
                header: Header::default(),
                claims,
            })
        } else {
            Err(ErrorKind::InvalidSignature.into())
        }
    } else {
        Err(ErrorKind::InvalidToken.into())
    }
}

pub fn get_claims(token: String) -> Result<Claims> {
    let b64: Vec<_> = token.split(".").collect();
    if b64.len() != 3 {
        return Err(ErrorKind::InvalidToken.into());
    }

    decode_payload(b64[1])
}

fn verify_signature(
    signature: String,
    signature_input: String,
    exp: i64,
    secret: String,
) -> Result<bool> {
    let full_secret = format!("{}_{}", secret, exp);
    verify(
        signature.as_ref(),
        signature_input.as_ref(),
        full_secret.as_ref(),
        Algorithm::HS256,
    )
}

fn decode_payload(payload_b64: &str) -> Result<Claims> {
    if let Ok(ref utf_bytes) = base64::decode_config(payload_b64, base64::URL_SAFE_NO_PAD) {
        if let Ok(user_data) = serde_json::from_slice::<Claims>(utf_bytes) {
            Ok(user_data)
        } else {
            Err(ErrorKind::InvalidToken.into())
        }
    } else {
        Err(ErrorKind::InvalidToken.into())
    }
}
