use sqlx::MySql;

use crate::{dao::app_dao, infra::utils::time};

pub async fn create_app<'e, E: sqlx::Executor<'e, Database = MySql>>(
    executor: E,
    user_id: usize,
    app_name: String,
) -> Result<u64, String> {
    let now = time::timestamp_secs();
    let res = app_dao::insert_app(executor, user_id as i64, app_name, now, now).await;

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
