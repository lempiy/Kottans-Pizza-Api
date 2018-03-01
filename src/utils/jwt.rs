use jwt::{encode, verify, Header, Algorithm, Validation, TokenData};
use uuid::Uuid;
use chrono::offset::Utc;
use chrono::Duration;
use jwt::errors::{Result, Error, ErrorKind};
use serde_json;
use base64;
use utils::types::StringError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: i64,
    pub username: String,
    pub uuid: Uuid,
}

pub fn generate(username: &str, uuid: Uuid)->Result<String> {
    let secret = "secret";
    let exp = (Utc::now() + Duration::hours(5)).naive_utc().timestamp();
    let full_secret = format!("{}_{}", secret, exp);
    let claims = Claims {
        exp,
        username: username.to_string(),
        uuid,
    };

    encode(&Header::default(), &claims, full_secret.as_ref())
}

pub fn check(token: String) -> Result<TokenData<Claims>> {
    let b64:Vec<_> = token.split(".").collect();
    if b64.len() != 3 {
        return Err(ErrorKind::InvalidToken.into())
    }

    let claims:Claims = match decode_payload(b64[1]) {
        Ok(claims) => claims,
        Err(e) => return Err(e)
    };

    if claims.exp < Utc::now().naive_utc().timestamp() {
        return Err(ErrorKind::ExpiredSignature.into())
    };

    if let Ok(valid) = verify_signature(
        b64[2].to_string(),
        (b64[0].to_owned()+"."+b64[1]), claims.exp) {
        if valid {
            Ok(TokenData{
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
    let b64:Vec<_> = token.split(".").collect();
    if b64.len() != 3 {
        return Err(ErrorKind::InvalidToken.into())
    }

    decode_payload(b64[1])
}

fn verify_signature(signature: String, signature_input: String, exp: i64) -> Result<bool> {
    let secret = "secret";
    let full_secret = format!("{}_{}", secret, exp);
    verify(signature.as_ref(),signature_input.as_ref(),
           full_secret.as_ref(), Algorithm::HS256)
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
