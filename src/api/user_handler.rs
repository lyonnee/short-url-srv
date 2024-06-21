use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

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

pub async fn registration(
    Json(req): Json<RegistrationReq>,
) -> impl IntoResponse {
    if req.email == Option::None && req.phone == Option::None{
       return  Json(Response::fail(1, "".to_string()))
    }

    if req.password.len() < 8{
        return  Json(Response::fail(1, "".to_string()))
    }

    user_logic::register_new(req.email, req.phone, req.password).await;

    Json(Response::ok(""))
}

pub async fn login(
    Json(req): Json<LoginReq>) -> impl IntoResponse {
    Json(Response::ok(""))
}
