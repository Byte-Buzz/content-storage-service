use crate::domain::{
    errors::RepositoryError,
    models::{PresignedGetRequest, PresignedPost, PresignedPostRequest},
};

#[async_trait::async_trait]
pub trait S3Interface: Send + Sync {
    fn create_presigned_post(
        &self,
        req: PresignedPostRequest,
    ) -> Result<PresignedPost, RepositoryError>;

    async fn presign_get(&self, req: PresignedGetRequest) -> Result<String, RepositoryError>;

    fn clone_box(&self) -> Box<dyn S3Interface + Send + Sync>;
}
