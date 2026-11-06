use std::str::FromStr;

use content_storage::content_storage_server::{ContentStorage, ContentStorageServer};
use css_core::domain::models;

use crate::app::{self, service::UploadService};

pub mod content_storage {
    tonic::include_proto!("content_storage");
}

pub struct ContentServiceGrpcHandler {
    uploads_service: UploadService,
    base_url: String,
}

impl ContentServiceGrpcHandler {
    pub fn new(app: &app::App) -> Self {
        Self {
            uploads_service: app.services.upload_service.clone(),
            base_url: app.config.server.base_url.clone(),
        }
    }
}

#[tonic::async_trait]
impl ContentStorage for ContentServiceGrpcHandler {
    async fn create_upload(
        &self,
        request: tonic::Request<content_storage::CreateUploadRequest>,
    ) -> Result<tonic::Response<content_storage::CreateUploadResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not implemented"))
    }

    async fn get_download_url(
        &self,
        request: tonic::Request<content_storage::GetDownloadUrlRequest>,
    ) -> Result<tonic::Response<content_storage::GetDownloadUrlResponse>, tonic::Status> {
        let id = uuid::Uuid::from_str(&request.get_ref().file_id).map_err(|e| {
            tracing::error!("Failed to parse file id: {}", e);
            tonic::Status::invalid_argument(e.to_string())
        })?;

        todo!("id: {}", id);
    }

    async fn change_file_visibility(
        &self,
        request: tonic::Request<content_storage::ChangeVisibilityRequest>,
    ) -> Result<tonic::Response<content_storage::ChangeVisibilityResponse>, tonic::Status> {
        todo!()
    }
}

pub fn create_grpc_service(app: &app::App) -> ContentStorageServer<ContentServiceGrpcHandler> {
    ContentStorageServer::new(ContentServiceGrpcHandler::new(app))
}
