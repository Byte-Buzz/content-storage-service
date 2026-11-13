use thiserror::Error;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("not found")]
    NotFound,
    #[error("already exists")]
    Conflict,
    #[error("backend unavailable")]
    Unavailable(#[source] BoxError),
    #[error("repository failure")]
    Internal(#[source] BoxError),
}
