use sqlx::MySql;

use super::ent;

pub async fn insert_user<'e, E>(
    executor: E,
    email: Option<String>,
    phone: Option<String>,
    salt: String,
    ciphertext: String,
) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'e, Database = MySql>,
{
    sqlx::query("INSERT INTO users (email,phone,salt,ciphertext,create_time,update_time) VALUES (?,?,?,?,?,?);")
    .bind(email)
    .bind(phone)
    .bind(salt)
    .bind(ciphertext)
    .execute(executor).await?;

    Ok(())
}

pub async fn query_user_by_phone<'e, E>(executor: E, phone: String) -> Result<ent::users::User, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = MySql>,
{
   return sqlx::query_as::<_,ent::users::User>("SELECT id,email,phone,salt,ciphertext,create_time,update_time FROM users WHERE phone=?").bind(phone).fetch_one(executor).await;
}

pub async fn query_user_by_email<'e, E>(executor: E, email: String) -> Result<ent::users::User, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = MySql>,
{
   return sqlx::query_as::<_,ent::users::User>("SELECT id,email,phone,salt,ciphertext,create_time,update_time FROM users WHERE email=?").bind(email).fetch_one(executor).await;
}
