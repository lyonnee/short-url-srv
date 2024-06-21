use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use super::response::Response;

#[derive(Deserialize)]
pub struct ShortenReq {
    pub original_url: String,
    pub expire_date: u32,
}

#[derive(Serialize)]
pub struct ShortenResp {
    short_key: String,
}

pub async fn shorten(
    Json(req): Json<ShortenReq>,
) -> impl IntoResponse {
    Json(Response::ok(req.original_url.to_uppercase()))
}

pub async fn redirect(
    Path(short_key): Path<String>,
) -> impl IntoResponse {
    Json(Response::ok(short_key.to_uppercase()))
}
