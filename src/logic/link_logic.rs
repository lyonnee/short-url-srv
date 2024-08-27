use std::io::Cursor;

use url::Url;

use crate::{dao::ent, infra::db, repository::link_repo};

pub async fn create_link(user_id: i64, app_id: i64, long_url: String) -> Result<String, String> {
    if is_valid_url(&long_url) == false {
        return Err(String::from("invalid url"));
    }

    if is_url_accessible(&long_url).await == false {
        return Err(String::from("url can not access"));
    }

    let res = murmur3::murmur3_32(&mut str_as_cursor(&long_url), 0);
    match res {
        Ok(hash) => {
            let short_key = base62::encode(hash);

            let mut tx = db::begin_db_transaction().await;

            let res = link_repo::save_new_link(&mut *tx, app_id, long_url, short_key.clone()).await;
            match res {
                Ok(_) => {}
                Err(e) => {
                    tracing::error!("{}", e);
                    return Err(e.to_string());
                }
            }

            match tx.commit().await {
                Ok(_) => Ok(short_key),
                Err(e) => {
                    tracing::error!("{}", e);
                    Err(e.to_string())
                }
            }
        }
        Err(e) => {
            tracing::error!("{}", e);
            Err(e.to_string())
        }
    }
}

pub async fn get_link_by_short_key(short_key: String) -> Option<ent::links::Link> {
    let mut conn = db::get_db_conn().await;
    link_repo::get_link_by_short_key(&mut *conn, short_key).await
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

fn str_as_cursor(string: &str) -> Cursor<&[u8]> {
    Cursor::new(string.as_bytes())
}
