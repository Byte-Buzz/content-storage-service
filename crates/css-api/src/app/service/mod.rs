pub mod error;

mod file_service;
mod upload_service;

use css_core::repository;
pub use file_service::FileService;
pub use upload_service::UploadService;

use crate::config;

#[derive(Clone)]
pub struct Services {
    pub upload_service: UploadService,
    pub file_service: FileService,
}

impl Services {
    pub fn new(repositories: repository::Repositories, config: config::Config) -> Self {
        let upload_service = UploadService::new(repositories.clone(), config.upload);
        let file_service = FileService::new(repositories.clone(), config.download);

        Self {
            upload_service,
            file_service,
        }
    }
}
