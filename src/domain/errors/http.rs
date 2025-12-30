pub struct HttpError {
    pub status: HttpErrorStatus,
    pub body: HttpErrorBody,
}

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
