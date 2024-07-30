use crate::{
    infra::db,
    repository::app_repo,
};

pub async fn create_app(user_id: usize, name: String) -> Result<u64, String> {
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
