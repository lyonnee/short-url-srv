use std::marker::PhantomData;

use sqlx::{Database, Executor, MySql, MySqlConnection, MySqlPool};

pub struct Context<E>
where
for<'a> E: sqlx::Executor<'a> + Send + Sync,
    {
    pub db: E,
}