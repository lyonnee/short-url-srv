use sqlx::Executor;

use crate::infra::db;

pub async fn inster_user<E> (
    executor:E,
    email: Option<String>,
    phone: Option<String>,
    salt: String,
    ciphertext: String,
)
 -> Result<(), sqlx::Error>
 where
for<'a> E: sqlx::Executor<'a> + Send + Sync,
{
    let result = sqlx::query("INSERT INTO users (email,phone,salt,ciphertext,create_time,update_time) VALUES (?,?,?,?,?,?)")
    .bind(email)
    .bind(phone)
    .bind(salt)
    .bind(ciphertext)
    .execute(&mut executor).await?;

    Ok(())
}
 