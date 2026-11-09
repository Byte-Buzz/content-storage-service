use std::time::{Duration, SystemTime};

use crate::app::service::error::ServiceError;

use super::content::content_storage as pb;
use css_core::domain::models;

fn task_from_proto(value: i32) -> Result<models::ProcessingTask, tonic::Status> {
    match pb::ProcessingTask::try_from(value) {
        Ok(pb::ProcessingTask::Thumbnails) => Ok(models::ProcessingTask::Thumbnails),
        Ok(pb::ProcessingTask::Frames) => Ok(models::ProcessingTask::Frames),
        Ok(pb::ProcessingTask::Unspecified) | Err(_) => Err(tonic::Status::invalid_argument(
            format!("unsupported task: {value}"),
        )),
    }
}

pub fn upload_request_from_proto(
    req: pb::CreateUploadRequest,
) -> Result<models::UploadRequest, tonic::Status> {
    let tasks = req
        .tasks
        .into_iter()
        .map(task_from_proto)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(models::UploadRequest {
        app_name: req.app_name,
        content_type: req.content_type,
        accepted_content_types: req.accepted_content_types,
        max_size: req.max_size,
        max_duration_seconds: req.max_duration_seconds,
        tasks,
        external_tasks: req.external_tasks,
        info: (!req.info.is_empty()).then_some(req.info),
        upload_url_ttl: req
            .upload_url_ttl
            .map(|s| Duration::from_secs(u64::from(s))),
    })
}

pub fn upload_response_to_proto(res: models::UploadResponse) -> pb::CreateUploadResponse {
    pb::CreateUploadResponse {
        file_id: res.file_id.to_string(),
        upload_url: res.upload_url,
        form_fields: res.form_fields,
        expires_at: Some(SystemTime::from(res.expires_at).into()),
    }
}

pub fn status_from_service_error(error: ServiceError) -> tonic::Status {
    match error {
        ServiceError::InvalidArgument(message) => tonic::Status::invalid_argument(message),
        e @ ServiceError::Unavailable(_) => tonic::Status::unavailable(e.to_string()),
        ServiceError::NotFound => tonic::Status::not_found("not found"),
        e @ ServiceError::Internal(_) => {
            tracing::error!(error = ?e, "request failed");
            tonic::Status::internal("internal error")
        }
    }
}
