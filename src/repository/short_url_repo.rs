use chrono::Utc;
use sqlx::MySql;

use crate::dao::{ent, short_url_dao};

pub async fn save_new_short_url<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    app_id: i64,
    origin_url: String,
    short_key: String,
) -> Result<u64, String> {
    let now = Utc::now().timestamp() as i32;
    let res =
        short_url_dao::insert_short_url(executor, app_id, origin_url, short_key, now, now).await;

    match res {
        Ok(res) => Ok(res.last_insert_id()),
        Err(e) => {
            tracing::error!("{e}");
            Err(e.to_string())
        }
    }
}

pub async fn get_short_url_by_key<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    short_key: String,
) -> Option<ent::short_urls::ShortUrl> {
    let res = short_url_dao::query_short_url_by_key(executor, short_key).await;

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
