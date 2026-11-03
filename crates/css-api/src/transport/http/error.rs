use crate::{app::service::error::ServiceError, domain::errors::RepositoryError};

pub struct HttpError {
    pub status: HttpErrorStatus,
    pub body: HttpErrorBody,
}

#[allow(dead_code)]
pub enum HttpErrorBody {
    Message(String),
    Json(serde_json::Value),
}

pub enum HttpErrorStatus {
    NotFound,
    BadRequest,
    PayloadTooLarge,
    Forbidden,
    InternalServerError,
}

#[derive(Debug, serde::Serialize)]
struct HttpErrorResponse<'a> {
    success: bool,
    error: &'a serde_json::Value,
}

impl HttpError {
    pub fn new(status: HttpErrorStatus, body: HttpErrorBody) -> Self {
        HttpError { status, body }
    }

    pub fn to_response(&self) -> actix_web::HttpResponse {
        let mut response = match self.status {
            HttpErrorStatus::NotFound => actix_web::HttpResponse::NotFound(),
            HttpErrorStatus::BadRequest => actix_web::HttpResponse::BadRequest(),
            HttpErrorStatus::PayloadTooLarge => actix_web::HttpResponse::PayloadTooLarge(),
            HttpErrorStatus::Forbidden => actix_web::HttpResponse::Forbidden(),
            HttpErrorStatus::InternalServerError => actix_web::HttpResponse::InternalServerError(),
        };

        let body = match &self.body {
            HttpErrorBody::Message(message) => HttpErrorResponse {
                success: false,
                error: &serde_json::json!({ "message": message }),
            },
            HttpErrorBody::Json(json) => HttpErrorResponse {
                success: false,
                error: json,
            },
        };

        match serde_json::to_string(&body) {
            Ok(body) => response.body(body),
            Err(err) => response.body(err.to_string()),
        }
    }
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
