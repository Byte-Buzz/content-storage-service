use chrono::NaiveDateTime;

use crate::domain::models::file::FileAccess;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Upload {
    pub id: uuid::Uuid,
    pub app_id: i64,
    pub filename: String,
    pub content_type: String,
    pub max_size: i32,
    pub settings: i32,
    pub info: serde_json::Value,
    pub access: FileAccess,
    pub secret: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct CreateUpload {
    pub id: uuid::Uuid,
    pub app_id: i64,
    pub filename: String,
    pub content_type: String,
    pub max_size: i32,
    pub settings: Option<i32>,
    pub info: serde_json::Value,
    pub access: FileAccess,
    pub secret: String,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone)]
pub struct CreateNewUpload {
    pub app_name: String,
    pub filename: String,
    pub settings: Option<i32>,
    pub content_type: String,
    pub max_size: u32,
    pub info: serde_json::Value,
    pub upload_url_ttl: Option<u32>,
    pub access: FileAccess,
}
