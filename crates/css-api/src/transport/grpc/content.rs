use crate::{
    app::{
        App,
        service::{FileService, UploadService},
    },
    transport::grpc::{
        content::content_storage::content_storage_service_server::{
            ContentStorageService, ContentStorageServiceServer,
        },
        convert,
    },
};

pub mod content_storage {
    tonic::include_proto!("content_storage.v1");
}

pub struct ContentServiceGrpcHandler {
    uploads_service: UploadService,
    file_service: FileService,
}

impl ContentServiceGrpcHandler {
    pub fn new(app: &App) -> Self {
        Self {
            uploads_service: app.services.upload_service.clone(),
            file_service: app.services.file_service.clone(),
        }
    }
}

#[tonic::async_trait]
impl ContentStorageService for ContentServiceGrpcHandler {
    async fn create_upload(
        &self,
        request: tonic::Request<content_storage::CreateUploadRequest>,
    ) -> Result<tonic::Response<content_storage::CreateUploadResponse>, tonic::Status> {
        let req = convert::upload_request_from_proto(request.into_inner())?;

        let created = self
            .uploads_service
            .create(req)
            .await
            .map_err(convert::status_from_service_error)?;

        Ok(tonic::Response::new(convert::upload_response_to_proto(
            created,
        )))
    }

    async fn get_download_urls(
        &self,
        request: tonic::Request<content_storage::GetDownloadUrlsRequest>,
    ) -> Result<tonic::Response<content_storage::GetDownloadUrlsResponse>, tonic::Status> {
        let req = request
            .into_inner()
            .items
            .into_iter()
            .map(convert::download_request_from_proto)
            .collect::<Result<Vec<_>, tonic::Status>>()?;

        let urls = self
            .file_service
            .get_download_urls(req)
            .await
            .map_err(convert::status_from_service_error)?;

        Ok(tonic::Response::new(
            content_storage::GetDownloadUrlsResponse {
                urls: urls
                    .into_iter()
                    .map(convert::download_response_to_proto)
                    .collect::<Vec<_>>(),
            },
        ))
    }
}

pub fn create_grpc_service(app: &App) -> ContentStorageServiceServer<ContentServiceGrpcHandler> {
    ContentStorageServiceServer::new(ContentServiceGrpcHandler::new(app))
}
