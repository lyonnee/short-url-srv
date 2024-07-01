use sqlx::MySql;

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
    sqlx::query("INSERT INTO users (email,phone,salt,ciphertext,create_time,update_time) VALUES (?,?,?,?,?,?)")
    .bind(email)
    .bind(phone)
    .bind(salt)
    .bind(ciphertext)
    .execute(executor).await?;

    Ok(())
}
