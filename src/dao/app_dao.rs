use sqlx::MySql;

pub async fn insert_app<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    user_id: i64,
    app_name: String,
    create_time: u64,
    update_time: u64,
) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
    sqlx::query("INSERT INTO apps (user_id,app_name,create_at,update_at) VALUES (?,?,?,?);")
        .bind(user_id)
        .bind(app_name)
        .bind(create_time)
        .bind(update_time)
        .execute(executor)
        .await
}
