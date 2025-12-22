use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("entity not found")]
    NotFound,

    #[error("conflict")]
    Conflict,

    #[error("invalid argument")]
    InvalidArgument,

    #[error("connection error")]
    Connection,

    #[error("query failed")]
    Query,

    #[error("internal repository error")]
    Internal,
}
