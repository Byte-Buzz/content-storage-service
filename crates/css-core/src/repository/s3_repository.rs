use std::time::SystemTime;

use aws_sdk_s3::presigning::PresigningConfig;
use chrono::Utc;

use crate::{
    domain::{
        errors::RepositoryError,
        interfaces::S3Interface,
        models::{PresignedGetRequest, PresignedPost, PresignedPostRequest},
    },
    infrastructure::{
        config::{S3Config, SigningCredentials},
        storage::{S3Client, sign_post_policy},
    },
};

pub struct S3Repository {
    s3_client: S3Client,
    presign_s3_client: S3Client,
    permanent_bucket: String,
    temp_bucket: String,
    credentials: SigningCredentials,
    public_endpoint: String,
}

impl S3Repository {
    pub fn new(s3_client: S3Client, presign_s3_client: S3Client, config: &S3Config) -> Self {
        Self {
            s3_client,
            presign_s3_client,
            permanent_bucket: config.bucket.clone(),
            temp_bucket: config.temp_bucket.clone(),
            credentials: config.credentials.clone(),
            public_endpoint: config.public_endpoint.clone(),
        }
    }
}

#[async_trait::async_trait]
impl S3Interface for S3Repository {
    fn create_presigned_post(
        &self,
        req: PresignedPostRequest,
    ) -> Result<PresignedPost, RepositoryError> {
        let now = Utc::now();
        let policy = sign_post_policy(&self.credentials, &self.temp_bucket, &req, now);

        let url = format!(
            "{}/{}",
            self.public_endpoint.trim_end_matches('/'),
            self.temp_bucket
        );

        Ok(PresignedPost {
            url,
            fields: policy,
            expires_at: req.expires_at,
        })
    }

    async fn presign_get(&self, req: PresignedGetRequest) -> Result<String, RepositoryError> {
        let config = PresigningConfig::builder()
            .start_time(SystemTime::from(req.signed_at))
            .expires_in(req.expires_in)
            .build()
            .map_err(|e| RepositoryError::Internal(e.into()))?;

        let mut get_object_builder = self
            .presign_s3_client
            .get_object()
            .bucket(&self.permanent_bucket)
            .key(&req.key);

        if let Some(name) = req.download_name {
            get_object_builder =
                get_object_builder.response_content_disposition(content_disposition(&name));
        }

        let presigned = get_object_builder
            .presigned(config)
            .await
            .map_err(|e| RepositoryError::Internal(e.into()))?;

        Ok(presigned.uri().to_string())
    }

    fn clone_box(&self) -> Box<dyn S3Interface + Send + Sync> {
        Box::new(S3Repository {
            s3_client: self.s3_client.clone(),
            presign_s3_client: self.presign_s3_client.clone(),
            permanent_bucket: self.permanent_bucket.clone(),
            temp_bucket: self.temp_bucket.clone(),
            credentials: self.credentials.clone(),
            public_endpoint: self.public_endpoint.clone(),
        })
    }
}

fn content_disposition(name: &str) -> String {
    let ascii: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || "._- ".contains(c) {
                c
            } else {
                '_'
            }
        })
        .collect();
    let encoded = percent_encoding::utf8_percent_encode(name, percent_encoding::NON_ALPHANUMERIC);
    format!("attachment; filename=\"{ascii}\"; filename*=UTF-8''{encoded}")
}
