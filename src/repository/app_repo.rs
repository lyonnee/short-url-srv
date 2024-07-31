use chrono::Utc;
use sqlx::MySql;

use crate::dao::{app_dao, ent};

pub async fn create_app<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    user_id: i64,
    app_name: String,
) -> Result<u64, String> {
    let now = Utc::now().timestamp() as i32;
    let res = app_dao::insert_app(executor, user_id, app_name, now, now).await;

    match res {
        Ok(res) => Ok(res.last_insert_id()),
        Err(e) => {
            tracing::error!("{e}");
            Err(e.to_string())
        }
    }
}

pub async fn get_app_list_by_user_id<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    user_id: i64,
    page: i32,
    size: i32,
) -> Option<Vec<ent::apps::App>> {
    let res = app_dao::page_query_apps(executor, user_id, page, size).await;

    match res {
        Ok(res) => Some(res),
        Err(e) => match e {
            sqlx::Error::RowNotFound => return None,
            _ => {
                tracing::error!("{}", e);
                return None;
            }
        },
    }
}
