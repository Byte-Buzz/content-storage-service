use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("entity not found")]
    NotFound,

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("connection error: {0}")]
    Connection(String),

    #[error("query failed: {0}")]
    Query(String),

    #[error("io error: {0}")]
    Io(std::io::Error),

    #[error("internal repository error: {0}")]
    Internal(String),
}
