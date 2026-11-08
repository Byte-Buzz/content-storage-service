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
