use std::sync::Arc;

use axum::{
    extract::{Json, State,Path},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use super::response::Response;

#[derive(Deserialize)]
pub struct CreateAppReq {
    pub app_name: String,
}

#[derive(Serialize)]
pub struct CreateAppResp {
    pub app_key: String,
}

pub async fn create_app(
    Json(req): Json<CreateAppReq>,
) -> impl IntoResponse {
    Json(Response::ok(""))
}
