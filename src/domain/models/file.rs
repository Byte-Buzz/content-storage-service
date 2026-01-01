use actix_multipart::Field;
use tokio::io::AsyncRead;

use crate::domain::models::Upload;

#[derive(Debug, Clone, serde::Serialize)]
pub struct FileInfo {
    pub key: String,
    pub filename: String,
}

#[derive(Debug, Clone, sqlx::FromRow, Default)]
pub struct File {
    pub id: uuid::Uuid,
    pub filename: String,
    pub app_id: i64,
    pub content_type: String,
    pub e_tag: String,
    pub access: FileAccess,
    pub miniatures: Vec<i32>,
    pub miniature_extension: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Default, sqlx::Type)]
#[sqlx(type_name = "file_access", rename_all = "snake_case")]
pub enum FileAccess {
    #[default]
    Private,
    Public,
}

pub struct TempFile {
    pub id: uuid::Uuid,
    pub filename: String,
    pub content_type: String,
    pub app_id: i64,
    pub created_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Default)]
pub struct CreateFile {
    pub id: uuid::Uuid,
    pub filename: String,
    pub app_id: i64,
    pub content_type: String,
    pub e_tag: String,
    pub access: Option<FileAccess>,
    pub miniatures: Vec<i32>,
    pub miniature_extension: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct CreateTempFile {
    pub id: uuid::Uuid,
    pub filename: String,
    pub content_type: String,
    pub app_id: i64,
    pub expires_at: Option<chrono::NaiveDateTime>,
}

pub struct UploadFile {
    pub upload: Upload,
    pub file: Field,
}

pub struct UploadFileRequest<'a> {
    pub id: uuid::Uuid,
    pub key: String,
    pub file: &'a mut (dyn AsyncRead + Unpin),
    pub content_type: String,
}

pub enum FileResponse {
    File(FileResponseFile),
    NotModified,
}

impl From<FileResponseFile> for FileResponse {
    fn from(value: FileResponseFile) -> Self {
        FileResponse::File(value)
    }
}

pub struct FileResponseFile {
    pub file: Box<dyn AsyncRead + Unpin>,
    pub content_type: String,
    pub filename: String,
    pub e_tag: String,
    pub size: Option<u32>,
}

pub struct FileSettings {
    settings: u32,
}

impl From<i32> for FileSettings {
    fn from(value: i32) -> Self {
        FileSettings {
            settings: value as u32,
        }
    }
}

impl FileSettings {
    pub fn is_temporary(&self) -> bool {
        self.check_bit(0)
    }

    pub fn need_miniatures(&self) -> bool {
        self.check_bit(1)
    }

    pub fn need_remove_metadata(&self) -> bool {
        self.check_bit(2)
    }

    pub fn need_temporary_storage(&self) -> bool {
        self.is_temporary() || self.need_miniatures() || self.need_remove_metadata()
    }

    fn check_bit(&self, bit: u32) -> bool {
        self.settings & (1 << bit) != 0
    }
}
