use chrono::Utc;

use crate::{
    domain::{
        errors::RepositoryError,
        interfaces::S3Interface,
        models::{PresignedPost, PresignedPostRequest},
    },
    infrastructure::{
        config::{S3Config, SigningCredentials},
        storage::{S3Client, sign_post_policy},
    },
};

pub struct S3Repository {
    s3_client: S3Client,
    permanent_bucket: String,
    temp_bucket: String,
    credentials: SigningCredentials,
    public_endpoint: String,
}

impl S3Repository {
    pub fn new(s3_client: S3Client, config: &S3Config) -> Self {
        Self {
            s3_client,
            permanent_bucket: config.bucket.clone(),
            temp_bucket: config.temp_bucket.clone(),
            credentials: config.credentials.clone(),
            public_endpoint: config.public_endpoint.clone(),
        }
    }
}

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

    fn clone_box(&self) -> Box<dyn S3Interface + Send + Sync> {
        Box::new(S3Repository {
            s3_client: self.s3_client.clone(),
            permanent_bucket: self.permanent_bucket.clone(),
            temp_bucket: self.temp_bucket.clone(),
            credentials: self.credentials.clone(),
            public_endpoint: self.public_endpoint.clone(),
        })
    }
}
