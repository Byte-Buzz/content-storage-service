use s3::serde_types::HeadObjectResult;
use tokio::io::AsyncRead;

use crate::{
    domain::{errors::RepositoryError, interfaces::S3Interface},
    infrastructure::storage::S3Client,
};

pub struct S3Repository {
    s3_client: S3Client,
}

impl S3Repository {
    pub fn new(s3_client: S3Client) -> Self {
        Self { s3_client }
    }
}

#[async_trait::async_trait(?Send)]
impl S3Interface for S3Repository {
    async fn upload_file(
        &self,
        file: &mut (dyn AsyncRead + Unpin),
        key: String,
    ) -> Result<Option<String>, RepositoryError> {
        let mut file = file;

        tracing::info!("uploading file to s3");

        let response = self
            .s3_client
            .bucket
            .put_object_stream(&mut file, &key)
            .await
            .map_err(|e| {
                tracing::error!("failed to upload file to s3: {}", e);
                RepositoryError::Internal(e.to_string())
            })?;

        tracing::info!("uploaded file to s3");
        tracing::debug!("response: {:#?}", response);

        match response.status_code() {
            200..=299 => (),
            _ => return Err(RepositoryError::Internal("upload failed".to_string())),
        };

        tracing::info!("upload complete");

        let response = self.s3_client.bucket.head_object(&key).await;

        tracing::debug!("response: {:#?}", response);

        match response {
            Ok(res) => Ok(res.0.e_tag),
            Err(e) => Err(RepositoryError::Internal(e.to_string())),
        }
    }

    fn clone_box(&self) -> Box<dyn S3Interface + Send + Sync> {
        Box::new(S3Repository {
            s3_client: self.s3_client.clone(),
        })
    }
}
