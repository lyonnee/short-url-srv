use chrono::Utc;
use sqlx::MySql;

use crate::dao::{ent::users::User, user_dao};

pub async fn save_new_user<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    email: Option<String>,
    phone: Option<String>,
    salt: String,
    ciphertext: String,
) -> Result<u64, String> {
    let now = Utc::now().timestamp() as i32;
    let res = user_dao::insert_user(executor, email, phone, salt, ciphertext, now, now).await;

    match res {
        Ok(res) => Ok(res.last_insert_id()),
        Err(e) => {
            tracing::error!("{e}");
            Err(e.to_string())
        }
    }
}

pub async fn get_user_by_phone_or_email<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    email: Option<String>,
    phone: Option<String>,
) -> Option<User> {
    if let Some(email) = email {
        let res = user_dao::query_user_by_email(executor, email).await;
        match res {
            Ok(user) => return Some(user),
            Err(e) => match e {
                sqlx::Error::RowNotFound => return None,
                _ => {
                    tracing::error!("{}", e);
                    return None;
                }
            },
        }
    }

    if let Some(phone) = phone {
        let res = user_dao::query_user_by_phone(executor, phone).await;
        match res {
            Ok(user) => return Some(user),
            Err(e) => match e {
                sqlx::Error::RowNotFound => return None,
                _ => {
                    tracing::error!("{}", e);
                    return None;
                }
            },
        }
    }

    return None;
}
