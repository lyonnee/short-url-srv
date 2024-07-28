use crate::infra::auth::jwt;
use axum::{
    extract::Request,
    http::{self, StatusCode},
    middleware::Next,
    response::Response,
};

async fn jwt_auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(uid) = verify(auth_header).await {
        // insert the current user into a request extension so the handler can
        // extract it
        req.extensions_mut().insert(uid);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn verify(auth_token: &str) -> Option<usize> {
    let res = jwt::verify(auth_token);
    match res {
        Ok(claims) => Some(claims.uid),
        Err(_) => None,
    }
}
