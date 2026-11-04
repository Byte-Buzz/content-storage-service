use css_core::{domain::interfaces, repository::Repositories};

pub struct FileService {
    upload_repository: Box<dyn interfaces::UploadInterface>,
    app_repository: Box<dyn interfaces::AppInterface>,
    s3_repository: Box<dyn interfaces::S3Interface>,
    file_repository: Box<dyn interfaces::FileInterface>,
}

impl FileService {
    pub fn new(repositories: Repositories) -> Self {
        Self {
            upload_repository: repositories.upload_repository,
            app_repository: repositories.app_repository,
            s3_repository: repositories.s3_repository,
            file_repository: repositories.file_repository,
        }
    }
}

impl Clone for FileService {
    fn clone(&self) -> Self {
        Self {
            upload_repository: self.upload_repository.clone_box(),
            app_repository: self.app_repository.clone_box(),
            s3_repository: self.s3_repository.clone_box(),
            file_repository: self.file_repository.clone_box(),
        }
    }
}
