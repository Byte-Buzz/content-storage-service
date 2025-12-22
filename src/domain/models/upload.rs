use chrono::NaiveDateTime;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Upload {
    pub id: uuid::Uuid,
    pub app_id: i64,
    pub settings: i32,
    pub info: serde_json::Value,
    pub secret: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct CreateUpload {
    pub id: uuid::Uuid,
    pub app_id: i64,
    pub settings: Option<i32>,
    pub info: serde_json::Value,
    pub secret: String,
    pub expires_at: Option<NaiveDateTime>,
}
