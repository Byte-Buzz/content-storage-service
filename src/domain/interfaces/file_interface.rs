use crate::domain::{errors::RepositoryError, models::File};

#[async_trait::async_trait]
pub trait FileInterface: Send + Sync {
    async fn create(&self, file: File) -> Result<File, RepositoryError>;

    fn clone_box(&self) -> Box<dyn FileInterface + Send + Sync>;
}
