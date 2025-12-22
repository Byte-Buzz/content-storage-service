use crate::domain::{
    errors::RepositoryError,
    models::{CreateUpload, Upload},
};

#[async_trait::async_trait]
pub trait UploadInterface {
    async fn create(&self, upload: CreateUpload) -> Result<u64, RepositoryError>;
    async fn get_by_id(&self, key: uuid::Uuid) -> Result<Upload, RepositoryError>;
    async fn delete(&self, key: uuid::Uuid) -> Result<u64, RepositoryError>;

    fn clone_box(&self) -> Box<dyn UploadInterface + Send + Sync>;
}
