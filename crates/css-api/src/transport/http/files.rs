use std::str::FromStr;

use actix_web::{
    HttpResponse, Responder, Result, get,
    http::header,
    web::{Data, Path, Query},
};
use tokio_util::io::ReaderStream;

use crate::{
    app::service::Services,
    domain::models::{FileResponse, PresignedQuery},
};

#[get("/{app}/{file_id}")]
async fn get_file(
    req: actix_web::HttpRequest,
    path: Path<(String, String)>,
    query: Query<PresignedQuery>,
    services: Data<Services>,
) -> Result<impl Responder> {
    let (app, file_id) = path.into_inner();

    let e_tag = req
        .headers()
        .get(header::IF_NONE_MATCH)
        .map(|v| v.to_str().ok())
        .flatten();

    let id = uuid::Uuid::from_str(&file_id).map_err(|e| {
        tracing::error!("Failed to parse file id: {}", e);
        actix_web::error::ErrorNotFound(e)
    })?;

    let file = services
        .file_service
        .get_file(&app, id, e_tag, query.0)
        .await?;

    match file {
        FileResponse::File(file) => {
            let mut response = HttpResponse::Ok();

            response
                .insert_header((header::ETAG, file.e_tag))
                .insert_header((header::CACHE_CONTROL, "max-age=31536000"))
                .content_type(file.content_type);

            if let Some(size) = file.size {
                response.insert_header((header::CONTENT_LENGTH, size.to_string()));
            }

            Ok(response.streaming(ReaderStream::new(file.file)))
        }
        FileResponse::NotModified => Ok(HttpResponse::NotModified().finish()),
    }
}
