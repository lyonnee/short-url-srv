use tracing::span::Id;
use url::Url;

use crate::{dao::ent, infra::db, repository::short_url_repo};

pub async fn create_short_url(
    user_id: i64,
    app_id: i64,
    origin_url: String,
) -> Result<String, String> {
    if is_valid_url(&origin_url) == false {
        return Err(String::from("invalid url"));
    }

    if is_url_accessible(&origin_url).await == false {
        return Err(String::from("url can not access"));
    }

    match snowflaker::next_id() {
        Ok(id) => {
            let short_key = base62::encode(id / 10000);

            let mut tx = db::begin_db_transaction().await;

            let res =
                short_url_repo::save_new_short_url(&mut *tx, app_id, origin_url, short_key.clone())
                    .await;

            match res {
                Ok(_) => {}
                Err(e) => {
                    // todo: 记录失败原因日志
                    return Err(e.to_string());
                }
            }

            match tx.commit().await {
                Ok(_) => Ok(short_key),
                Err(e) => {
                    // todo: 记录失败原因日志
                    Err(e.to_string())
                }
            }
        }
        Err(e) => {
            return Err(e.to_string());
        }
    }
}

pub async fn get_origin_url_by_key(short_key: String) -> Option<ent::short_urls::ShortUrl> {
    let mut conn = db::get_db_conn().await;
    short_url_repo::get_short_url_by_key(&mut *conn, short_key).await
}

fn is_valid_url(url: &str) -> bool {
    Url::parse(url).is_ok()
}

async fn is_url_accessible(url: &str) -> bool {
    match reqwest::get(url).await {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}
