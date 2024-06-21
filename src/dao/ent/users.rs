use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct User {
    pub id: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub salt: Option<String>,
    pub passtext: Option<String>,
    pub create_time:Option<u32>,
    pub update_time:Option<u32>,
}
