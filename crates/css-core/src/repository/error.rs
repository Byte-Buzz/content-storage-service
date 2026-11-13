use crate::domain::errors::RepositoryError;

pub fn map_sqlx_error(err: sqlx::Error) -> RepositoryError {
    match err {
        sqlx::Error::RowNotFound => RepositoryError::NotFound,
        sqlx::Error::Database(ref db) if db.is_unique_violation() => RepositoryError::Conflict,
        e @ (sqlx::Error::Io(_)
        | sqlx::Error::Tls(_)
        | sqlx::Error::PoolTimedOut
        | sqlx::Error::PoolClosed) => RepositoryError::Unavailable(e.into()),
        e => RepositoryError::Internal(e.into()),
    }
}
