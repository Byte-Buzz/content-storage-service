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
    async fn create_new_upload(
        &self,
        request: tonic::Request<uploads::CreateNewUploadRequest>,
    ) -> Result<tonic::Response<uploads::CreateNewUploadResponse>, tonic::Status> {
        let data = models::CreateNewUpload {
            app_name: request.get_ref().app_name.clone(),
            settings: request.get_ref().settings,
            info: serde_json::from_str(request.get_ref().info_json.as_str())
                .map_err(|e| tonic::Status::internal(format!("Failed to parse info: {}", e)))?,
            expires_at: request
                .get_ref()
                .expires_at
                .map(|v| chrono::DateTime::from_timestamp(v as i64, 0).map(|v| v.naive_local()))
                .flatten(),
        };

        let upload = self
            .uploads_service
            .create(data)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(tonic::Response::new(uploads::CreateNewUploadResponse {
            upload_url: format!(
                "{}upload/{}?secret={}&expires_at={}",
                self.base_url,
                upload.id,
                upload.secret,
                upload.expires_at.and_utc().timestamp(),
            ),
        }))
    }
}

pub fn create_grpc_service(app: &app::App) -> UploadsServer<UploadGrpcHandler> {
    UploadsServer::new(UploadGrpcHandler::new(app))
}
