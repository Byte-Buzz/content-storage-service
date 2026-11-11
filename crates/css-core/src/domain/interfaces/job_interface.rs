use crate::domain::{errors::RepositoryError, models::ProcessingJobKind};

#[async_trait::async_trait]
pub trait JobInterface: Send + Sync {
    async fn enqueue(
        &self,
        file_id: uuid::Uuid,
        kind: ProcessingJobKind,
    ) -> Result<bool, RepositoryError>;

    fn clone_box(&self) -> Box<dyn JobInterface + Send + Sync>;
}
