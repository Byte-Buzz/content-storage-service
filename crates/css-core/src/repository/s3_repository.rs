use crate::{
    domain::interfaces::S3Interface,
    infrastructure::{config::S3Config, storage::S3Client},
};

pub struct S3Repository {
    s3_client: S3Client,
    permanent_bucket: String,
    temp_bucket: String,
}

impl S3Repository {
    pub fn new(s3_client: S3Client, config: &S3Config) -> Self {
        Self {
            s3_client,
            permanent_bucket: config.bucket.clone(),
            temp_bucket: config.temp_bucket.clone(),
        }
    }
}

#[async_trait::async_trait(?Send)]
impl S3Interface for S3Repository {
    fn clone_box(&self) -> Box<dyn S3Interface + Send + Sync> {
        Box::new(S3Repository {
            s3_client: self.s3_client.clone(),
            permanent_bucket: self.permanent_bucket.clone(),
            temp_bucket: self.temp_bucket.clone(),
        })
    }
}
