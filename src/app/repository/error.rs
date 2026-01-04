use crate::domain::errors::RepositoryError;

pub fn map_sqlx_error(err: sqlx::Error) -> RepositoryError {
    match err {
        sqlx::Error::RowNotFound => RepositoryError::NotFound,
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            RepositoryError::Conflict(e.to_string())
        }
        sqlx::Error::InvalidArgument(e) => RepositoryError::InvalidArgument(e),
        sqlx::Error::Io(e) => RepositoryError::Connection(e.to_string()),
        sqlx::Error::Tls(e) => RepositoryError::Connection(e.to_string()),
        sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed => {
            RepositoryError::Connection("pool closed".to_string())
        }
        sqlx::Error::Database(e) => RepositoryError::Query(e.to_string()),
        e => RepositoryError::Internal(e.to_string()),
    }
}
