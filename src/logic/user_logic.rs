use bcrypt::{DEFAULT_COST,hash_with_salt,verify};
use rand::{RngCore,rngs::OsRng};

use crate::{infra::db, repository::user_repo};

pub async fn register_new(
    email: Option<String>,
    phone: Option<String>,
    password: String,
) -> Result<u64,String>{
    let mut tx = db::begin_db_transaction().await;
    let user = user_repo::find_user_by_phone_or_email(&mut *tx,email.clone(),phone.clone()).await;
    if let Some(_) = user{
        let _ = tx.rollback().await;
        return  Err(String::from("email or phone has used"));
    }

    let salt =  generate_random_bytes();
    let hashed_pwd = hash_with_salt(password, DEFAULT_COST, salt).unwrap().to_string();


    let salt_str = String::from_utf8_lossy(&salt);
    let res = user_repo::create_user( &mut *tx,email,phone, salt_str.to_string(),hashed_pwd).await;

    let commit_res = tx.commit().await;

    match commit_res{
        Ok(_)=> Ok(res.unwrap()),
        Err(err) => {
            // todo: 记录失败原因日志
            Err(err.to_string())
        },
    }
}

pub async fn login(
    email: Option<String>,
    phone: Option<String>,
    password: String,
) -> bool{
    let mut conn = db::get_db_conn().await;
    let user = user_repo::find_user_by_phone_or_email(&mut *conn,email,phone).await;

    if let Some(user) = user{
        let hash = user.ciphertext.unwrap();
        let res = verify(password,&hash);

        if let Ok(pass) = res {
            return pass;
        }

        // TODO: 记录错误res.error
        return false;
    }

    return  false;
}

fn generate_random_bytes() -> [u8; 16] {
    let mut rdm_bytes = [0u8; 16];
    OsRng.fill_bytes(&mut rdm_bytes);
    rdm_bytes
}
