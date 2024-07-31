use crate::{dao::ent, infra::db, repository::app_repo};

pub async fn create_app(user_id: i64, name: String) -> Result<u64, String> {
    let mut tx = db::begin_db_transaction().await;
    let res = app_repo::create_app(&mut *tx, user_id, name).await;

    let commit_res = tx.commit().await;

    match commit_res {
        Ok(_) => Ok(res.unwrap()),
        Err(err) => {
            // todo: 记录失败原因日志
            Err(err.to_string())
        }
    }
}

pub async fn get_user_app_list(user_id: i64, page: i32, size: i32) -> Option<Vec<ent::apps::App>> {
    let mut conn = db::get_db_conn().await;
    app_repo::get_app_list_by_user_id(&mut *conn, user_id, page, size).await
}
