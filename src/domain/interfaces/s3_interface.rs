use tokio::io::AsyncRead;

use crate::domain::errors::RepositoryError;

#[async_trait::async_trait(?Send)]
pub trait S3Interface: Send + Sync {
    async fn upload_file(
        &self,
        file: &mut (dyn AsyncRead + Unpin),
        key: String,
    ) -> Result<Option<String>, RepositoryError>;

    async fn max_file_size(&self) -> u64 {
        200 * 1024 * 1024
    }

    fn clone_box(&self) -> Box<dyn S3Interface + Send + Sync>;
}
