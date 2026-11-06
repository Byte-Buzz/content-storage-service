use chrono::Utc;

use crate::domain::models::file::FileVisibility;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Upload {
    pub id: uuid::Uuid,
    pub app_id: i64,
    pub content_type: String,
    pub accepted_content_types: Vec<String>,
    pub max_size: i64,
    pub max_duration_seconds: Option<i32>,
    pub tasks: Vec<ProcessingTask>,
    pub external_tasks: Vec<String>,
    pub info: Option<Vec<u8>>,
    pub created_at: chrono::DateTime<Utc>,
    pub expires_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "processing_task", rename_all = "snake_case")]
pub enum ProcessingTask {
    Thumbnails,
    Frames,
}

#[derive(Debug, Clone)]
pub struct CreateUpload {
    pub id: uuid::Uuid,
    pub app_id: i64,
    pub content_type: String,
    pub accepted_content_types: Vec<String>,
    pub max_size: i64,
    pub max_duration_seconds: Option<i32>,
    pub tasks: Vec<ProcessingTask>,
    pub external_tasks: Vec<String>,
    pub info: Option<Vec<u8>>,
    pub expires_at: chrono::DateTime<Utc>,
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
    pub access: FileVisibility,
}
