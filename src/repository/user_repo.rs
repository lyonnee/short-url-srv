use sqlx::MySql;

use crate::{
    dao::{ent::users::User, user_dao},
    infra::utils::time,
};

pub async fn create_user<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    email: Option<String>,
    phone: Option<String>,
    salt: String,
    ciphertext: String,
) -> Result<u64, String> {
    let now = time::timestamp_secs();
    let res = user_dao::insert_user(executor, email, phone, salt, ciphertext, now, now).await;

    match res {
        Ok(res) => {
            tracing::info!("inset res: {:?}", res);
            Ok(res.last_insert_id())
        }
        Err(err) => {
            tracing::error!("{err}");
            Err(err.to_string())
        }
    }
}

pub async fn find_user_by_phone_or_email<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    email: Option<String>,
    phone: Option<String>,
) -> Option<User> {
    if let Some(email) = email {
        let res = user_dao::query_user_by_email(executor, email).await;
        match res {
            Ok(user) => return Some(user),
            Err(err) => match err {
                sqlx::Error::RowNotFound => return None,
                _ => {
                    tracing::error!("{}", err);
                    return None;
                }
            },
        }
    }

    if let Some(phone) = phone {
        let res = user_dao::query_user_by_phone(executor, phone).await;
        match res {
            Ok(user) => return Some(user),
            Err(err) => match err {
                sqlx::Error::RowNotFound => return None,
                _ => {
                    tracing::error!("{}", err);
                    return None;
                }
            },
        }
    }

    return None;
}
