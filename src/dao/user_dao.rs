use sqlx::MySql;

use super::ent;

pub async fn insert_user<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    email: Option<String>,
    phone: Option<String>,
    salt: String,
    ciphertext: String,
    create_time: i32,
    update_time: i32,
) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
    sqlx::query(
        "INSERT INTO users (email,phone,salt,ciphertext,create_at,update_at) VALUES (?,?,?,?,?,?);",
    )
    .bind(email)
    .bind(phone)
    .bind(salt)
    .bind(ciphertext)
    .bind(create_time)
    .bind(update_time)
    .execute(executor)
    .await
}

pub async fn query_user_by_phone<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    phone: String,
) -> Result<ent::users::User, sqlx::Error> {
    sqlx::query_as::<_, ent::users::User>(
        "SELECT id,email,phone,salt,ciphertext,create_at,update_at FROM users WHERE phone=?",
    )
    .bind(phone)
    .fetch_one(executor)
    .await
}

pub async fn query_user_by_email<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    email: String,
) -> Result<ent::users::User, sqlx::Error> {
    sqlx::query_as::<_, ent::users::User>(
        "SELECT id,email,phone,salt,ciphertext,create_at,update_at FROM users WHERE email=?",
    )
    .bind(email)
    .fetch_one(executor)
    .await
}
