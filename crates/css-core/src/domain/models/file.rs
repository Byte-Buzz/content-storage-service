use chrono::Utc;

#[derive(Debug, Clone, sqlx::FromRow, Default)]
pub struct File {
    pub id: uuid::Uuid,
    pub app_id: i64,
    pub content_type: String,
    pub variants: Vec<String>,
    pub visibility: FileVisibility,
    pub visibility_version: i64,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, Default, sqlx::Type)]
#[sqlx(type_name = "file_visibility", rename_all = "snake_case")]
pub enum FileVisibility {
    #[default]
    Private,
    Public,
}

#[derive(Debug, Clone, Default)]
pub struct CreateFile {
    pub id: uuid::Uuid,
    pub app_id: i64,
    pub content_type: String,
    pub variants: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct FileForDownload {
    pub id: uuid::Uuid,
    pub app_id: i64,
    pub app_name: String,
    pub content_type: String,
    pub variants: Vec<String>,
}
