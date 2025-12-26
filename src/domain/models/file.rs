use actix_multipart::Field;
use mime_guess::Mime;

use crate::domain::models::Upload;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub key: String,
    pub filename: String,
}

#[derive(Debug, Clone, sqlx::FromRow, Default)]
pub struct File {
    pub id: uuid::Uuid,
    pub name: String,
    pub app_id: uuid::Uuid,
    pub extension: String,
    pub info: serde_json::Value,
    pub access: FileAccess,
    pub miniatures: Option<Vec<u32>>,
    pub hash: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Default)]
pub enum FileAccess {
    #[default]
    Signed,
    Unsigned,
}

pub struct UploadFile {
    pub upload: Upload,
    pub file: Field,
}
