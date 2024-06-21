use sqlx::MySql;

use super::repo::Context;
use crate::infra::db;

pub async fn create_user<E>(
    ctx: &mut Context<E>,
    email: Option<String>,
    phone: Option<String>,
    salt: String,
    ciphertext: String,
) where
    for<'a> E: sqlx::Executor<'a> + Send + Sync,
{
}
