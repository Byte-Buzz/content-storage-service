use thiserror::Error;

use css_core::domain::errors::RepositoryError;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("repository error: {0}")]
    RepositoryError(RepositoryError),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("forbidden")]
    Forbidden,

    #[error("internal service error: {0}")]
    Internal(String),
}

impl From<RepositoryError> for ServiceError {
    fn from(err: RepositoryError) -> Self {
        ServiceError::RepositoryError(err)
    }
}
