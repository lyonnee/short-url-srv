use chrono::Utc;
use sqlx::MySql;

use crate::dao::{ent, links_dao};

pub async fn save_new_link<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    app_id: i64,
    long_url: String,
    short_key: String,
) -> Result<u64, String> {
    let now = Utc::now().timestamp() as i32;
    let res = links_dao::insert_link(executor, app_id, long_url, short_key, now, now).await;

    match res {
        Ok(res) => Ok(res.last_insert_id()),
        Err(e) => {
            tracing::error!("{e}");
            Err(e.to_string())
        }
    }
}

pub async fn get_link_by_short_key<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    short_key: String,
) -> Option<ent::links::Link> {
    let res = links_dao::query_link_by_key(executor, short_key).await;

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
