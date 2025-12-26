use crate::domain::errors::RepositoryError;

pub fn map_sqlx_error(err: sqlx::Error) -> RepositoryError {
    match err {
        sqlx::Error::RowNotFound => RepositoryError::NotFound,
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => RepositoryError::Conflict,
        sqlx::Error::InvalidArgument(_) => RepositoryError::InvalidArgument,
        sqlx::Error::Io(_)
        | sqlx::Error::Tls(_)
        | sqlx::Error::PoolTimedOut
        | sqlx::Error::PoolClosed => RepositoryError::Connection,
        sqlx::Error::Database(_) => RepositoryError::Query,
        e => RepositoryError::Internal(e.to_string()),
    }
}
