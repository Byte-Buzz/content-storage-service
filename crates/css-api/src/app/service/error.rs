use thiserror::Error;

use css_core::domain::errors::RepositoryError;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("{0}")]
    InvalidArgument(String),
    #[error("not found")]
    NotFound,
    #[error("temporarily unavailable")]
    Unavailable(#[source] RepositoryError),
    #[error("internal error")]
    Internal(#[source] RepositoryError),
}

impl From<RepositoryError> for ServiceError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::NotFound => Self::NotFound,
            e @ RepositoryError::Unavailable(_) => Self::Unavailable(e),
            e => Self::Internal(e),
        }
    }
}
