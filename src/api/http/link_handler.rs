use axum::{
    extract::{Json, Path},
    response::{IntoResponse, Redirect},
    Extension,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::logic::link_logic;

use super::{middleware::jwt::Claims, response::Response};

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
    let res = link_logic::create_link(user.uid as i64, req.app_id, req.original_url).await;
    match res {
        Ok(id) => Json(Response::ok(id)),
        Err(e) => Json(Response::fail(1, e)),
    }
}

pub async fn redirect(Path(short_key): Path<String>) -> impl IntoResponse {
    let res = link_logic::get_link_by_short_key(short_key).await;

    match res {
        Some(link) => {
            let redirect = Redirect::to(&link.long_url);
            return redirect.into_response();
        }
        None => {
            let json_str = serde_json::json!(Response::ok(()));
            (StatusCode::OK, json_str.to_string()).into_response()
        }
    }
}
