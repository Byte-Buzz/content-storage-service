use crate::domain::{
    errors::RepositoryError,
    models::{CreateUpload, Upload},
};

#[async_trait::async_trait]
pub trait UploadInterface: Send + Sync {
    async fn create(&self, upload: CreateUpload) -> Result<Upload, RepositoryError>;
    async fn get_by_id(&self, key: uuid::Uuid) -> Result<Upload, RepositoryError>;
    async fn delete(&self, key: uuid::Uuid) -> Result<u64, RepositoryError>;

    fn clone_box(&self) -> Box<dyn UploadInterface + Send + Sync>;
}
