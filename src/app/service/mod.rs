mod file_service;
mod upload_service;

pub use file_service::FileService;
pub use upload_service::UploadService;

use crate::app::repository;

#[derive(Clone)]
pub struct Services {
    pub upload_service: UploadService,
    pub file_service: FileService,
}

impl Services {
    pub fn new(repositories: repository::Repositories) -> Self {
        let upload_service = UploadService::new(
            repositories.upload_repository,
            repositories.app_repository.clone_box(),
        );

        let file_service = FileService::new(
            repositories.app_repository,
            repositories.s3_repository,
            repositories.file_repository,
        );

        Self {
            upload_service,
            file_service,
        }
    }
}
