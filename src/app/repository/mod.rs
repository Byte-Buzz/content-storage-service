mod error;

mod app_repository;
mod file_repository;
mod s3_repository;
mod upload_repository;

use aws_sdk_s3::Client;

use crate::{domain::interfaces, infrastructure::config::S3Config};

pub(self) use self::error::map_sqlx_error;

pub struct Repositories {
    pub upload_repository: Box<dyn interfaces::UploadInterface>,
    pub app_repository: Box<dyn interfaces::AppInterface>,
    pub file_repository: Box<dyn interfaces::FileInterface>,
    pub s3_repository: Box<dyn interfaces::S3Interface>,
}

impl Repositories {
    pub fn new(pool: sqlx::PgPool, s3_client: Client, config: &S3Config) -> Self {
        Self {
            upload_repository: Box::new(upload_repository::UploadRepository::new(pool.clone())),
            app_repository: Box::new(app_repository::AppRepository::new(pool.clone())),
            file_repository: Box::new(file_repository::FileRepository::new(pool.clone())),
            s3_repository: Box::new(s3_repository::S3Repository::new(s3_client, config)),
        }
    }
}

impl Clone for Repositories {
    fn clone(&self) -> Self {
        Self {
            upload_repository: self.upload_repository.clone_box(),
            app_repository: self.app_repository.clone_box(),
            file_repository: self.file_repository.clone_box(),
            s3_repository: self.s3_repository.clone_box(),
        }
    }
}
