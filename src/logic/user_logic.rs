use std::{string, sync::Arc};

use bcrypt::{hash,DEFAULT_COST,hash_with_salt};
use rand::{RngCore,rngs::OsRng};
use sqlx::Acquire;

use crate::{infra::db, repository::{repo,user_repo}};

pub async fn register_new(
    email: Option<String>,
    phone: Option<String>,
    password: String,
) {
    let salt =  generate_random_bytes();
    let hashed_pwd = hash_with_salt(password, DEFAULT_COST, salt).unwrap().to_string();

    let mut conn = db::begin_db_transaction();

    let result = sqlx::query("INSERT INTO users (email,phone,salt,ciphertext,create_time,update_time) VALUES (?,?,?,?,?,?)")
    .bind(email)
    .bind(phone)
    .bind(String::from_utf8(salt.to_vec()).unwrap())
    .bind(hashed_pwd)
    .fetch_one(&mut *conn).await;
    // let mut ctx = repo::Context{
    //     db: &conn
    // };

    // user_repo::create_user(&mut ctx,email, phone, String::from_utf8(salt.to_vec()).unwrap(),hashed_pwd);
}

fn generate_random_bytes() -> [u8; 16] {
    let mut bytes = [0u8; 16];
    OsRng.fill_bytes(&mut bytes);
    bytes
}
