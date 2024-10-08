use bcrypt::{hash_with_salt, verify, DEFAULT_COST};
use rand::{rngs::OsRng, RngCore};

use crate::{infra::db, repository::user_repo};

pub async fn register_new(
    email: Option<String>,
    phone: Option<String>,
    password: String,
) -> Result<u64, String> {
    let mut tx = db::begin_db_transaction().await;
    let user = user_repo::get_user_by_phone_or_email(&mut *tx, email.clone(), phone.clone()).await;
    if let Some(_) = user {
        let _ = tx.rollback().await;
        return Err(String::from("email or phone has used"));
    }

    let salt = generate_random_bytes();
    let hashed_pwd = hash_with_salt(password, DEFAULT_COST, salt)
        .unwrap()
        .to_string();

    let salt_str = String::from_utf8_lossy(&salt);
    let res =
        user_repo::save_new_user(&mut *tx, email, phone, salt_str.to_string(), hashed_pwd).await;

    let commit_res = tx.commit().await;

    match commit_res {
        Ok(_) => Ok(res.unwrap()),
        Err(e) => {
            tracing::error!("{}", e);
            Err(e.to_string())
        }
    }
}

pub async fn login(
    email: Option<String>,
    phone: Option<String>,
    password: String,
) -> Result<usize, String> {
    let mut conn = db::get_db_conn().await;
    let res = user_repo::get_user_by_phone_or_email(&mut *conn, email, phone).await;

    match res {
        Some(user) => {
            let hash = user.ciphertext.unwrap();

            let verify_res = verify(password, &hash);
            match verify_res {
                Ok(pass) => match pass {
                    true => Ok(user.id.unwrap() as usize),
                    false => Err(String::from("the password is incorrect")),
                },
                Err(e) => {
                    tracing::error!("{}", e);
                    Err(format!("{}", e))
                }
            }
        }
        None => Err(String::from("user not found")),
    }
}

fn generate_random_bytes() -> [u8; 16] {
    let mut rdm_bytes = [0u8; 16];
    OsRng.fill_bytes(&mut rdm_bytes);
    rdm_bytes
}
