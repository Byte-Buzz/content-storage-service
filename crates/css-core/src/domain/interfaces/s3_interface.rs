use crate::domain::{
    errors::RepositoryError,
    models::{PresignedPost, PresignedPostRequest},
};

pub trait S3Interface: Send + Sync {
    fn create_presigned_post(
        &self,
        req: PresignedPostRequest,
    ) -> Result<PresignedPost, RepositoryError>;

    fn clone_box(&self) -> Box<dyn S3Interface + Send + Sync>;
}
