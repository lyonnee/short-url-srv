use sqlx::MySql;

use super::ent;

// pub struct ShortUrl {
//     pub id: Option<i64>,
//     pub app_id: i64,
//     pub origin_url: String,
//     pub short_key: String,
//     pub hits: i64,
//     pub create_at: Option<i32>,
//     pub update_at: Option<i32>,
// }

pub async fn insert_short_url<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    app_id: i64,
    origin_url: String,
    short_key: String,
    create_time: i32,
    update_time: i32,
) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
    sqlx::query("INSERT INTO short_urls (app_id,origin_url,short_key,hits,create_at,update_at) VALUES (?,?,?,?,?,?);")
        .bind(app_id)
        .bind(origin_url)
        .bind(short_key)
        .bind(0)
        .bind(create_time)
        .bind(update_time)
        .execute(executor)
        .await
}

pub async fn query_short_url_by_key<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    short_key: String,
) -> Result<ent::short_urls::ShortUrl, sqlx::Error> {
    sqlx::query_as::<_, ent::short_urls::ShortUrl>(
        "SELECT * FROM `short_urls` WHERE `short_key` = ?;",
    )
    .bind(short_key)
    .fetch_one(executor)
    .await
}
