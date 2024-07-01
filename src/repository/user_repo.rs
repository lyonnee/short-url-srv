use sqlx::MySql;

use crate::dao::user_dao;

pub async fn create_user<'e,E>(
    executor: E,
    email: Option<String>,
    phone: Option<String>,
    salt: String,
    ciphertext: String,
)  where
    E: sqlx::Executor<'e, Database = MySql>,
{
   user_dao::insert_user(executor, email, phone, salt, ciphertext);
}
