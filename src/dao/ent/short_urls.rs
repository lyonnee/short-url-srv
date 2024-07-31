use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct ShortUrl {
    pub id: Option<i64>,
    pub app_id: i64,
    pub origin_url: String,
    pub short_key: String,
    pub hits: i64,
    pub create_at: Option<i32>,
    pub update_at: Option<i32>,
}
