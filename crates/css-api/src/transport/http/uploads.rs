use std::str::FromStr;

use actix_multipart::Multipart;
use actix_web::{
    HttpResponse, Responder, post,
    web::{Data, Path, Query},
};
use futures_util::StreamExt;

use crate::{
    app::service::Services,
    domain::{
        errors::{HttpError, HttpErrorBody, HttpErrorStatus},
        models::{SecretQuery, UploadFile},
    },
};

#[post("/upload/{id}")]
async fn upload_file(
    id: Path<String>,
    query: Query<SecretQuery>,
    services: Data<Services>,
    mut files: Multipart,
) -> impl Responder {
    let id = uuid::Uuid::from_str(&id);

    if id.is_err() {
        return HttpError::new(
            HttpErrorStatus::BadRequest,
            HttpErrorBody::Message("invalid uuid".to_string()),
        )
        .to_response();
    }

    let id = id.unwrap();
    if id.is_nil() {
        return HttpError::new(
            HttpErrorStatus::BadRequest,
            HttpErrorBody::Message("invalid uuid".to_string()),
        )
        .to_response();
    }

    let upload = match services
        .upload_service
        .check_upload(id, &query.secret)
        .await
    {
        Ok(upload) => upload,
        Err(err) => return err.to_response(),
    };

    let file = match files.next().await {
        Some(Ok(field)) => field,
        Some(Err(err)) => {
            return HttpError::new(
                HttpErrorStatus::InternalServerError,
                HttpErrorBody::Message(err.to_string()),
            )
            .to_response();
        }
        None => {
            return HttpError::new(
                HttpErrorStatus::BadRequest,
                HttpErrorBody::Message("no file".to_string()),
            )
            .to_response();
        }
    };

    let file_info = services
        .file_service
        .upload_file(UploadFile { upload, file })
        .await;

    if let Err(err) = file_info {
        return err.to_response();
    }

    let file_info = file_info.unwrap();

    HttpResponse::Created().json(file_info)
}
