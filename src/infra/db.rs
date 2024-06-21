use std::sync::Arc;

use sqlx::{
    migrate::MigrateDatabase,
    mysql::{MySqlPool, MySqlPoolOptions, MySqlRow},
};
use tokio::sync::OnceCell;

static DB_POOL: OnceCell<Arc<MySqlPool>> = OnceCell::const_new();

pub async fn init(db_url: &str, max_conns: u32) {
    if sqlx::MySql::database_exists(db_url).await.unwrap_or(false) == false {}

    let db = MySqlPoolOptions::new()
        .max_connections(max_conns)
        .connect(&db_url)
        .await
        .expect("Failed to create pool");

        sqlx::migrate!().run(&db).await;

    DB_POOL.set(Arc::new(db));
}

pub async fn get_db_conn() -> sqlx::pool::PoolConnection<sqlx::MySql> {
    DB_POOL.get().unwrap().acquire().await.unwrap()
}

pub async fn begin_db_transaction() -> sqlx::Transaction<'static, sqlx::MySql> {
    DB_POOL.get().unwrap().begin().await.unwrap()
}