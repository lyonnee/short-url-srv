use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct App {
    pub id: Option<i64>,
    pub user_id: i64,
    pub app_name: String,
    pub create_at: Option<i32>,
    pub update_at: Option<i32>,
}
