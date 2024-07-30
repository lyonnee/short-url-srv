use chrono::{Duration, Utc};
use once_cell::sync::Lazy;

use axum::{
    extract::Request,
    http::{self, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{errors::Error, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

static KEYS: Lazy<Keys> = Lazy::new(|| Keys::new("".as_bytes(), "".as_bytes()));
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(encoding_key: &[u8], decoding_key: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(encoding_key),
            decoding: DecodingKey::from_secret(decoding_key),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub uid: usize,  // user id
    pub iat: usize,  // Issued at (as UTC timestamp)
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub jti: usize, // Optional. jwt id
    pub iss: String, // Optional. Issuer
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
    pub access_token: String,
    pub token_type: String,
}

pub fn authorization(user_id: usize, token_id: usize) -> Result<TokenPayload, Error> {
    let now = Utc::now();
    let iat: usize = now.timestamp() as usize;

    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;

    let claims = Claims {
        uid: user_id,
        iat,
        exp,
        jti: token_id,
        iss: String::from("auther"),
    };

    let token = jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding)?;

    Ok(TokenPayload {
        access_token: token,
        token_type: "Bearer".to_string(),
    })
}

pub(crate) async fn authentication(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let verify_res = jsonwebtoken::decode(auth_header, &KEYS.decoding, &Validation::default())
        .map(|data: jsonwebtoken::TokenData<Claims>| data.claims);

    match verify_res {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(next.run(req).await)
        }
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
