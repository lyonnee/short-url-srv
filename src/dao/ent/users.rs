use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct User {
    pub id: Option<i64>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub salt: Option<String>,
    pub ciphertext: Option<String>,
    pub create_at: Option<i32>,
    pub update_at: Option<i32>,
}
