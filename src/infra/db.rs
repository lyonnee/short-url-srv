use std::sync::Arc;

use sqlx::{
    migrate::MigrateDatabase,
    mysql::{MySqlPool, MySqlPoolOptions},
};
use tokio::sync::OnceCell;

use super::config;

static DB_POOL: OnceCell<Arc<MySqlPool>> = OnceCell::const_new();

pub async fn init() {
    let lock_config = config::get_configs();
    let config = lock_config.as_ref().unwrap();

    if sqlx::MySql::database_exists(&config.mysql.dsn).await.unwrap_or(false) == false {}

    let db = MySqlPoolOptions::new()
        .max_connections(config.mysql.max_conns)
        .connect(&config.mysql.dsn)
        .await
        .expect("Failed to create pool");

    let _ = sqlx::migrate!().run(&db).await;

    let _ = DB_POOL.set(Arc::new(db));

    tracing::info!("The database connect has been initialized!!!");
}

pub async fn get_db_conn() -> sqlx::pool::PoolConnection<sqlx::MySql> {
    DB_POOL.get().unwrap().acquire().await.unwrap()
}

pub async fn begin_db_transaction() -> sqlx::Transaction<'static, sqlx::MySql> {
    DB_POOL.get().unwrap().begin().await.unwrap()
}
