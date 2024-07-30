use chrono::{Duration, Utc};
use once_cell::sync::Lazy;

use axum::{
    body::Body, extract::Request, http::{self, StatusCode}, middleware::Next, response::{IntoResponse, Response}, Json
};
use jsonwebtoken::{errors::Error, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;

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
    pub iss: String, // Optional. Issuer
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
    pub access_token: String,
    pub token_type: String,
}

pub fn authorization(user_id: usize) -> Result<TokenPayload, Error> {
    let now = Utc::now();
    let iat: usize = now.timestamp() as usize;

    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;

    let claims = Claims {
        uid: user_id,
        iat,
        exp,
        iss: String::from("auther"),
    };

    let token = jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding)?;

    Ok(TokenPayload {
        access_token: token,
        token_type: "Bearer".to_string(),
    })
}

pub struct AuthError {
    message: String,
    status_code: StatusCode,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<Body> {
        let body = Json(json!({
            "error": self.message,
        }));

        (self.status_code, body).into_response()
    }
}

pub(crate) async fn authentication(mut req: Request, next: Next) -> Result<Response<Body>, AuthError> {
    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| AuthError {
            message: "Empty header is not allowed".to_string(),
            status_code: StatusCode::FORBIDDEN
        })?,
        None => return Err(AuthError {
            message: "Please add the JWT token to the header".to_string(),
            status_code: StatusCode::FORBIDDEN
        }),
    };

    let mut header = auth_header.split_whitespace();
    let (_bearer, token) = (header.next(), header.next());

    let verify_res = jsonwebtoken::decode(token.unwrap(), &KEYS.decoding, &Validation::default())
        .map(|data: jsonwebtoken::TokenData<Claims>| data.claims);

    match verify_res {
        Ok(claims) => {
            let now = Utc::now().timestamp() as usize;

            if claims.iat > now {

            }

            if claims.exp < now {
                
            }

            req.extensions_mut().insert(claims);
            Ok(next.run(req).await)
        }
        Err(e) => {
            tracing::error!("{}", e);
            Err(AuthError {
                message: "Unable to decode token".to_string(),
                status_code: StatusCode::UNAUTHORIZED
            })
        }
    }
}
