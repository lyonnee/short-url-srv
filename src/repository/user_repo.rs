use sqlx::MySql;

use crate::dao::{ent::users::User, user_dao};

pub async fn create_user<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    email: Option<String>,
    phone: Option<String>,
    salt: String,
    ciphertext: String,
)
{
    user_dao::insert_user(executor, email, phone, salt, ciphertext);
}

pub async fn find_user_by_phone_or_email<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    email: Option<String>,
    phone: Option<String>,
) -> Result<User,()>
{
    if let Some(email) = email {
        let res = user_dao::query_user_by_email(executor, email).await;
        return Ok(res.unwrap());
    }

    if let Some(phone) = phone {
        let res = user_dao::query_user_by_phone(executor, phone).await;
        return Ok(res.unwrap());
    }

    return Err(());
}
