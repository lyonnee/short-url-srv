use sqlx::MySql;

use super::ent;

pub async fn insert_link<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    app_id: i64,
    long_url: String,
    short_key: String,
    create_time: i32,
    update_time: i32,
) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
    sqlx::query("INSERT INTO links (app_id,long_url,short_key,hits,create_at,update_at) VALUES (?,?,?,?,?,?);")
        .bind(app_id)
        .bind(long_url)
        .bind(short_key)
        .bind(0)
        .bind(create_time)
        .bind(update_time)
        .execute(executor)
        .await
}

pub async fn query_link_by_key<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    short_key: String,
) -> Result<ent::links::Link, sqlx::Error> {
    sqlx::query_as::<_, ent::links::Link>("SELECT * FROM `links` WHERE `short_key` = ?;")
        .bind(short_key)
        .fetch_one(executor)
        .await
}
