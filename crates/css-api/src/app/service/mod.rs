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
        let upload_service = UploadService::new(repositories.clone());
        let file_service = FileService::new(repositories.clone());

        Self {
            upload_service,
            file_service,
        }
    }
}
