use axum::{extract::Json, response::IntoResponse};
use serde::Deserialize;

use crate::logic::user_logic;

use super::response::Response;

#[derive(Deserialize)]
pub struct RegistrationReq {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginReq {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: String,
}

pub async fn registration(Json(req): Json<RegistrationReq>) -> impl IntoResponse {
    if req.email == Option::None && req.phone == Option::None {
        return Json(Response::fail(
            1,
            "password or email cannot both be empty".to_string(),
        ));
    }

    if req.password.len() < 8 {
        return Json(Response::fail(1, "password length too short".to_string()));
    }

    let res = user_logic::register_new(req.email, req.phone, req.password).await;

    match res {
        Ok(id) => Json(Response::ok(id)),
        Err(e) => Json(Response::fail(1, e)),
    }
}

pub async fn login(Json(req): Json<LoginReq>) -> impl IntoResponse {
    if req.email == Option::None && req.phone == Option::None {
        return Json(Response::fail(
            1,
            "password or email cannot both be empty".to_string(),
        ));
    }

    if req.password.len() < 8 {
        return Json(Response::fail(1, "password length too short".to_string()));
    }

    let res = user_logic::login(req.email, req.phone, req.password).await;

    match res {
        Ok(token) => Json(Response::ok(token)),
        Err(e) => Json(Response::fail(1, e)),
    }
}
