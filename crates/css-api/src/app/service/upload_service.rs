use css_core::{
    domain::{interfaces, models},
    repository::Repositories,
};

use crate::app::service::error::ServiceError;

pub struct UploadService {
    upload_repository: Box<dyn interfaces::UploadInterface>,
    app_repository: Box<dyn interfaces::AppInterface>,
}

impl UploadService {
    pub fn new(repositories: Repositories) -> Self {
        Self {
            upload_repository: repositories.upload_repository,
            app_repository: repositories.app_repository,
        }
    }

    pub async fn create(
        &self,
        upload: models::CreateNewUpload,
    ) -> Result<models::Upload, ServiceError> {
        Err(ServiceError::Internal("Not implemented".to_string()))
    }
}

impl Clone for UploadService {
    fn clone(&self) -> Self {
        Self {
            upload_repository: self.upload_repository.clone_box(),
            app_repository: self.app_repository.clone_box(),
        }
    }
}
