use axum::{
    extract::{Json, Path},
    handler::Handler,
    response::{IntoResponse, Redirect},
    Extension,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::logic::short_url_logic;

use super::{
    middleware::jwt::Claims,
    response::{self, Response},
};

#[derive(Deserialize)]
pub struct ShortenReq {
    pub app_id: i64,
    pub original_url: String,
    pub expire_date: u32,
}

#[derive(Serialize)]
pub struct ShortenResp {
    short_key: String,
}

pub async fn shorten(
    Extension(user): Extension<Claims>,
    Json(req): Json<ShortenReq>,
) -> impl IntoResponse {
    let res =
        short_url_logic::create_short_url(user.uid as i64, req.app_id, req.original_url).await;
    match res {
        Ok(id) => Json(Response::ok(id)),
        Err(e) => Json(Response::fail(1, e)),
    }
}

pub async fn redirect(Path(short_key): Path<String>) -> impl IntoResponse {
    let res = short_url_logic::get_origin_url_by_key(short_key).await;

    match res {
        Some(short_url) => {
            let redirect = Redirect::to(&short_url.origin_url);
            return redirect.into_response();
        }
        None => {
            let json_str = serde_json::json!(Response::ok(()));
            (StatusCode::OK, json_str.to_string()).into_response()
        }
    }
}
