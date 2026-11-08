use std::collections::HashMap;

use aws_sigv4::sign::v4::{calculate_signature, generate_signing_key};
use base64::{Engine, engine::general_purpose::STANDARD};
use chrono::{DateTime, Utc};
use serde_json::json;

use crate::{domain::models::PresignedPostRequest, infrastructure::config::SigningCredentials};

pub fn sign_post_policy(
    credentials: &SigningCredentials,
    bucket: &str,
    req: &PresignedPostRequest,
    now: DateTime<Utc>,
) -> HashMap<String, String> {
    let expiration = req.expires_at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
    let date = now.format("%Y%m%d").to_string();
    let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();
    let credential = format!(
        "{}/{}/{}/s3/aws4_request",
        &credentials.access_key, date, &credentials.region
    );

    let policy = json!({
        "expiration": expiration,
        "conditions": [
            {"bucket": bucket},
            {"key": &req.key},
            {"Content-Type": &req.content_type},
            ["content-length-range", 1, &req.max_size],
            {"x-amz-algorithm": "AWS4-HMAC-SHA256"},
            {"x-amz-credential": credential},
            {"x-amz-date": amz_date}
        ]
    });
    let b64_policy = STANDARD.encode(policy.to_string());

    let key = generate_signing_key(
        &credentials.secret_key,
        now.into(),
        &credentials.region,
        "s3",
    );
    let signature = calculate_signature(key, b64_policy.as_bytes());

    HashMap::from([
        ("key".to_string(), req.key.clone()),
        ("Content-Type".to_string(), req.content_type.clone()),
        (
            "x-amz-algorithm".to_string(),
            "AWS4-HMAC-SHA256".to_string(),
        ),
        ("x-amz-credential".to_string(), credential),
        ("x-amz-date".to_string(), amz_date),
        ("policy".to_string(), b64_policy),
        ("x-amz-signature".to_string(), signature),
    ])
}
