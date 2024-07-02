use core::str;
use std::{  string, sync::Arc,error::Error};

use bcrypt::{hash,DEFAULT_COST,hash_with_salt,verify};
use rand::{RngCore,rngs::OsRng};
use sqlx::Acquire;

use crate::{infra::db, repository::{user_repo}};

pub async fn register_new(
    email: Option<String>,
    phone: Option<String>,
    password: String,
) {
    let salt =  generate_random_bytes();
    let hashed_pwd = hash_with_salt(password, DEFAULT_COST, salt).unwrap().to_string();

    let mut tx = db::begin_db_transaction().await;

    user_repo::create_user( &mut *tx,email, phone, String::from_utf8(salt.to_vec()).unwrap(),hashed_pwd).await;

    tx.commit().await;
}

pub async fn login(
    email: Option<String>,
    phone: Option<String>,
    password: String,
) -> bool{
    let mut conn = db::get_db_conn().await;
    let user = user_repo::find_user_by_phone_or_email(&mut *conn,email,phone).await;

    if let Ok(user) = user{
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
    let mut bytes = [0u8; 16];
    OsRng.fill_bytes(&mut bytes);
    bytes
}
