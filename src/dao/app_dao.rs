use sqlx::MySql;

use super::ent;

pub async fn insert_app<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    user_id: i64,
    app_name: String,
    create_time: i32,
    update_time: i32,
) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
    sqlx::query("INSERT INTO apps (user_id,app_name,create_at,update_at) VALUES (?,?,?,?);")
        .bind(user_id)
        .bind(app_name)
        .bind(create_time)
        .bind(update_time)
        .execute(executor)
        .await
}

pub async fn page_query_apps<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    user_id: i64,
    page: i32,
    size: i32,
) -> Result<Vec<ent::apps::App>, sqlx::Error> {
    sqlx::query_as::<_, ent::apps::App>(
        "SELECT * FROM `apps` WHERE `user_id` = ? ORDER BY `id` LIMIT ? OFFSET ?;",
    )
    .bind(user_id)
    .bind(size)
    .bind((page - 1) * size)
    .fetch_all(executor)
    .await
}
