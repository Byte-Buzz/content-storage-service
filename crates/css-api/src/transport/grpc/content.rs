use std::str::FromStr;

use css_core::domain::models;

use crate::{
    app::{self, service::UploadService},
    transport::grpc::content::content_storage::content_storage_service_server::{
        ContentStorageService, ContentStorageServiceServer,
    },
};

pub mod content_storage {
    tonic::include_proto!("content_storage.v1");
}

pub struct ContentServiceGrpcHandler {
    uploads_service: UploadService,
}

impl ContentServiceGrpcHandler {
    pub fn new(app: &app::App) -> Self {
        Self {
            uploads_service: app.services.upload_service.clone(),
        }
    }
}

#[tonic::async_trait]
impl ContentStorageService for ContentServiceGrpcHandler {
    async fn create_upload(
        &self,
        request: tonic::Request<content_storage::CreateUploadRequest>,
    ) -> Result<tonic::Response<content_storage::CreateUploadResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not implemented"))
    }

    async fn get_download_urls(
        &self,
        request: tonic::Request<content_storage::GetDownloadUrlsRequest>,
    ) -> Result<tonic::Response<content_storage::GetDownloadUrlsResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not implemented"))
    }
}

pub fn create_grpc_service(
    app: &app::App,
) -> ContentStorageServiceServer<ContentServiceGrpcHandler> {
    ContentStorageServiceServer::new(ContentServiceGrpcHandler::new(app))
}
