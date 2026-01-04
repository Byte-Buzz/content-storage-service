use std::str::FromStr;

use content_storage::content_storage_server::{ContentStorage, ContentStorageServer};

use crate::{
    app::{self, service::UploadService},
    domain::models,
};

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
        let data = models::CreateNewUpload {
            app_name: request.get_ref().app_name.clone(),
            filename: request.get_ref().filename.clone(),
            settings: request.get_ref().settings.map(|v| v as i32),
            content_type: request.get_ref().content_type.clone(),
            max_size: request.get_ref().max_size,
            info: serde_json::from_str(request.get_ref().info_json.as_str()).map_err(|e| {
                tracing::error!("Failed to parse info: {}", e);
                tonic::Status::invalid_argument(format!("Failed to parse info: {}", e))
            })?,
            upload_url_ttl: request.get_ref().upload_url_ttl,
            access: match request.get_ref().visibility() {
                content_storage::FileVisibility::Public => models::FileAccess::Public,
                _ => models::FileAccess::Private,
            },
        };

        let upload = self.uploads_service.create(data).await.map_err(|e| {
            tracing::error!("Failed to create upload: {}", e);
            tonic::Status::internal(e.to_string())
        })?;

        Ok(tonic::Response::new(
            content_storage::CreateUploadResponse {
                file_id: upload.id.to_string(),
                upload_url: format!(
                    "{}upload/{}?secret={}&expires={}",
                    self.base_url,
                    upload.id,
                    upload.secret,
                    upload.expires_at.and_utc().timestamp(),
                ),
                upload_expires_at: upload.expires_at.and_utc().timestamp() as u64,
            },
        ))
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
