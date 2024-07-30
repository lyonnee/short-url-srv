use axum::{extract::Json, response::IntoResponse, Extension};
use serde::{Deserialize, Serialize};

use crate::logic::app_logic;

use super::{middleware::jwt::Claims, response::Response};

#[derive(Deserialize)]
pub struct CreateAppReq {
    pub app_name: Option<String>,
}

#[derive(Serialize)]
pub struct CreateAppResp {
    pub app_key: String,
}

pub async fn create_app(
    Extension(user): Extension<Claims>,
    Json(req): Json<CreateAppReq>,
) -> impl IntoResponse {
    if req.app_name == Option::None {
        return Json(Response::fail(
            1,
            "password or email cannot both be empty".to_string(),
        ));
    }

    let res = app_logic::create_app(user.uid, req.app_name.unwrap()).await;

    match res {
        Ok(id) => Json(Response::ok(id)),
        Err(e) => Json(Response::fail(1, e)),
    }
}
