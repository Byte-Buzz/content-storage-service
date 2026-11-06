use crate::domain::{errors::RepositoryError, models};

/// Interface for file repository operations.
#[async_trait::async_trait]
pub trait FileInterface: Send + Sync {
    async fn get_by_id(&self, id: uuid::Uuid) -> Result<models::File, RepositoryError>;

    /// Returns a boxed clone of the implementor for dynamic dispatch.
    fn clone_box(&self) -> Box<dyn FileInterface + Send + Sync>;
}
