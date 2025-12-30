use tokio::io::AsyncRead;

use crate::domain::{errors::RepositoryError, models::UploadFileRequest};

#[async_trait::async_trait(?Send)]
pub trait S3Interface: Send + Sync {
    async fn upload_file(
        &self,
        request: UploadFileRequest<'_>,
    ) -> Result<Option<String>, RepositoryError>;

    async fn upload_temp_file(
        &self,
        request: UploadFileRequest<'_>,
    ) -> Result<Option<String>, RepositoryError>;

    async fn max_file_size(&self) -> u32 {
        2 * 1024 * 1024 * 1024
    }

    fn clone_box(&self) -> Box<dyn S3Interface + Send + Sync>;
}
