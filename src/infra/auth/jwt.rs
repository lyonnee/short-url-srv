use axum::{async_trait, extract::FromRequestParts, http::StatusCode};
use jsonwebtoken::{errors::Error, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::infra::utils::time;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: usize, // Issued at (as UTC timestamp)
    pub iss: String, // Optional. Issuer
    pub uid: usize, // user id
    pub jti: usize, // Optional. jwt id
}

impl Claims {
    pub fn new(uid: usize) -> Self {
        let iat = (time::timestamp_millis()) as usize;
        let exp = iat + 10;

        Self {
            iat: iat,
            exp: exp,
            iss: String::from(""),
            uid: uid,
            jti: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
    pub access_token: String,
    pub token_type: String,
}

pub fn create_token(uid: usize) -> Result<TokenPayload, Error> {
    let encoding_key: EncodingKey = EncodingKey::from_secret("".as_bytes());
    let token = sign(uid, &encoding_key)?;

    Ok(TokenPayload {
        access_token: token,
        token_type: "Bearer".to_string(),
    })
}

pub fn verify(token: &str) -> Result<Claims, Error> {
    let decode_key = &DecodingKey::from_secret("".as_bytes());

    Ok(
        jsonwebtoken::decode(token, decode_key, &Validation::default())
            .map(|data: jsonwebtoken::TokenData<Claims>| data.claims)?,
    )
}

fn sign(uid: usize, encoding_key: &EncodingKey) -> Result<String, Error> {
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(uid),
        encoding_key,
    )?)
}
