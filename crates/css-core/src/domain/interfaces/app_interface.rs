use crate::domain::{errors::RepositoryError, models};

#[async_trait::async_trait]
pub trait AppInterface: Send + Sync {
    async fn create_if_not_exists(&self, name: &str) -> Result<models::App, RepositoryError>;

    fn clone_box(&self) -> Box<dyn AppInterface + Send + Sync>;
}
