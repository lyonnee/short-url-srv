use axum::{extract::{Json, Path}, response::IntoResponse, Extension};
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

    let res = app_logic::create_app(user.uid as i64, req.app_name.unwrap()).await;

    match res {
        Ok(id) => Json(Response::ok(id)),
        Err(e) => Json(Response::fail(1, e)),
    }
}

#[derive(Serialize)]
pub struct App{
    pub id: i64,
    pub name: String,
    pub create_at: i32,
}
pub async fn get_app_list( Extension(user): Extension<Claims>, Path((page,size)): Path<(i32,i32)>) -> impl IntoResponse {
    let res = app_logic::get_user_app_list(user.uid as i64, page, size).await;

    let mut app_list = Vec::new();
    match res{
        Some(apps) => {
            for app in apps.into_iter() {
                app_list.push(App{id:app.id.unwrap(),name:app.app_name,create_at:app.create_at.unwrap()})
            }
            
        },
        None => (),
    }
    Json(Response::ok(app_list))
}