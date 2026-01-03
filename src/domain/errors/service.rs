use thiserror::Error;

use crate::domain::errors::{HttpError, HttpErrorBody, HttpErrorStatus, RepositoryError};

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("repository error: {0}")]
    RepositoryError(crate::domain::errors::RepositoryError),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("forbidden")]
    Forbidden,

    #[error("internal service error: {0}")]
    Internal(String),
}

impl ServiceError {
    pub fn to_response(&self) -> actix_web::HttpResponse {
        match self {
            ServiceError::RepositoryError(e) => match e {
                RepositoryError::NotFound => HttpError::new(
                    HttpErrorStatus::NotFound,
                    HttpErrorBody::Message("not found".to_string()),
                ),
                RepositoryError::Io(e) => match e.kind() {
                    std::io::ErrorKind::FileTooLarge => HttpError::new(
                        HttpErrorStatus::PayloadTooLarge,
                        HttpErrorBody::Message("file too large".to_string()),
                    ),
                    _ => HttpError::new(
                        HttpErrorStatus::InternalServerError,
                        HttpErrorBody::Message(e.to_string()),
                    ),
                },
                _ => HttpError::new(
                    HttpErrorStatus::InternalServerError,
                    HttpErrorBody::Message(e.to_string()),
                ),
            },
            ServiceError::BadRequest(message) => HttpError::new(
                HttpErrorStatus::BadRequest,
                HttpErrorBody::Message(message.to_string()),
            ),
            ServiceError::Forbidden => HttpError::new(
                HttpErrorStatus::Forbidden,
                HttpErrorBody::Message("forbidden".to_string()),
            ),
            ServiceError::Internal(message) => HttpError::new(
                HttpErrorStatus::InternalServerError,
                HttpErrorBody::Message(message.to_string()),
            ),
        }
        .to_response()
    }
}

impl From<crate::domain::errors::RepositoryError> for ServiceError {
    fn from(err: crate::domain::errors::RepositoryError) -> Self {
        ServiceError::RepositoryError(err)
    }
}

impl From<ServiceError> for actix_web::Error {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::BadRequest(e) => actix_web::error::ErrorBadRequest(e),
            ServiceError::Forbidden => actix_web::error::ErrorForbidden("forbidden"),
            ServiceError::RepositoryError(e) => match e {
                RepositoryError::Io(e) => match e.kind() {
                    std::io::ErrorKind::FileTooLarge => {
                        actix_web::error::ErrorPayloadTooLarge(e.to_string())
                    }
                    _ => actix_web::error::ErrorInternalServerError(e.to_string()),
                },
                RepositoryError::NotFound => actix_web::error::ErrorNotFound(e.to_string()),
                _ => actix_web::error::ErrorInternalServerError(e.to_string()),
            },
            _ => actix_web::error::ErrorInternalServerError(value.to_string()),
        }
    }
}
