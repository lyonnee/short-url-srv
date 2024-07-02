use sqlx::MySql;

use crate::dao::{ent::users::User, user_dao};

pub async fn create_user<'e, E>(
    executor: E,
    email: Option<String>,
    phone: Option<String>,
    salt: String,
    ciphertext: String,
) where
    E: sqlx::Executor<'e, Database = MySql>,
{
    user_dao::insert_user(executor, email, phone, salt, ciphertext);
}

pub async fn find_user_by_phone_or_email<'e, E>(
    executor: E,
    email: Option<String>,
    phone: Option<String>,
) -> Result<User,&'static str>
where
    E: sqlx::Executor<'e, Database = MySql>,
{
    if let Some(email) = email {
        let res = user_dao::query_user_by_email(executor, email).await;
        if let Ok(user) = res {
            return Ok(user);
        }

        return Err("");
    }

    if let Some(phone) = phone {
        let res = user_dao::query_user_by_phone(executor, phone).await;
        if let Ok(user) = res {
            return Ok(user);
        }

        return Err("");
    }

    return Err("");
}
