use crate::domain::{errors::RepositoryError, models};

/// Interface for file repository operations.
///
/// **Architecture note**:
/// The methods `create_file_and_delete_upload` and `create_temp_file_and_delete_upload` operate on multiple tables
/// (`files`/`temp_files` and `uploads`). This slightly violates the Single Responsibility Principle (one repository
/// per entity/table).
///
/// We accept this trade-off because:
/// - These operations are always atomic and part of the same business transaction.
/// - Splitting them would require either passing transactions through traits or adding a separate service layer,
///   which adds unnecessary boilerplate at this stage.
/// - In the future, these can be moved to a dedicated use-case/service if needed.
///
/// For now, this is a pragmatic compromise.
#[async_trait::async_trait]
pub trait FileInterface: Send + Sync {
    /// Creates a record in `files` and atomically deletes the related record from `uploads`.
    ///
    /// All operations occur in a single transaction.
    async fn create_file_and_delete_upload(
        &self,
        file: models::CreateFile,
    ) -> Result<models::File, RepositoryError>;

    /// Creates a record in `temp_files` and atomically deletes the related record from `uploads`.
    ///
    /// All operations occur in a single transaction.
    async fn create_temp_file_and_delete_upload(
        &self,
        file: models::CreateTempFile,
    ) -> Result<models::TempFile, RepositoryError>;

    async fn get_by_id(&self, id: uuid::Uuid) -> Result<models::File, RepositoryError>;

    /// Returns a boxed clone of the implementor for dynamic dispatch.
    fn clone_box(&self) -> Box<dyn FileInterface + Send + Sync>;
}
