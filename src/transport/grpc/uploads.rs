use std::str::FromStr;

use uploads::uploads_server::{Uploads, UploadsServer};

use crate::{
    app::{self, service::UploadService},
    domain::models,
};

pub mod uploads {
    tonic::include_proto!("uploads");
}

pub struct UploadGrpcHandler {
    uploads_service: UploadService,
    base_url: String,
}

impl UploadGrpcHandler {
    pub fn new(app: &app::App) -> Self {
        Self {
            uploads_service: app.services.upload_service.clone(),
            base_url: app.config.server.base_url.clone(),
        }
    }
}

#[tonic::async_trait]
impl Uploads for UploadGrpcHandler {
    async fn create_upload(
        &self,
        request: tonic::Request<uploads::CreateUploadRequest>,
    ) -> Result<tonic::Response<uploads::CreateUploadResponse>, tonic::Status> {
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
                uploads::FileVisibility::Public => models::FileAccess::Public,
                _ => models::FileAccess::Private,
            },
        };

        let upload = self.uploads_service.create(data).await.map_err(|e| {
            tracing::error!("Failed to create upload: {}", e);
            tonic::Status::internal(e.to_string())
        })?;

        Ok(tonic::Response::new(uploads::CreateUploadResponse {
            file_id: upload.id.to_string(),
            upload_url: format!(
                "{}upload/{}?secret={}&expires={}",
                self.base_url,
                upload.id,
                upload.secret,
                upload.expires_at.and_utc().timestamp(),
            ),
            upload_expires_at: upload.expires_at.and_utc().timestamp() as u64,
        }))
    }

    async fn get_download_url(
        &self,
        request: tonic::Request<uploads::GetDownloadUrlRequest>,
    ) -> Result<tonic::Response<uploads::GetDownloadUrlResponse>, tonic::Status> {
        let id = uuid::Uuid::from_str(&request.get_ref().file_id).map_err(|e| {
            tracing::error!("Failed to parse file id: {}", e);
            tonic::Status::invalid_argument(e.to_string())
        })?;

        todo!("id: {}", id);
    }
}

pub fn create_grpc_service(app: &app::App) -> UploadsServer<UploadGrpcHandler> {
    UploadsServer::new(UploadGrpcHandler::new(app))
}
