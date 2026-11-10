use std::collections::HashMap;

use chrono::{DateTime, Utc};

pub struct PresignedPostRequest {
    pub key: String,
    pub content_type: String,
    pub max_size: u64,
    pub expires_at: DateTime<Utc>,
}

pub struct PresignedPost {
    pub url: String,
    pub fields: HashMap<String, String>,
    pub expires_at: DateTime<Utc>,
}

pub struct DownloadUrlRequest {
    pub file_id: uuid::Uuid,
    pub variant: Option<String>,
    pub download_name: Option<String>,
}

pub struct PresignedGetRequest {
    pub key: String,
    pub download_name: Option<String>,
    pub signed_at: DateTime<Utc>,
    pub expires_in: std::time::Duration,
}

pub struct PresignedGet {
    pub file_id: uuid::Uuid,
    pub variant: Option<String>,
    pub url: String,
    pub expires_at: DateTime<Utc>,
}
