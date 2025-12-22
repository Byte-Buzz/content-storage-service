mod error;
mod upload_repository;

use crate::{domain::interfaces, infrastructure::config::Config};

pub(self) use self::error::map_sqlx_error;

pub use self::upload_repository::UploadRepository;

pub struct Repositories {
    pub upload_repository: Box<dyn interfaces::UploadInterface>,
}

impl Repositories {
    pub fn new(pool: sqlx::PgPool, s3_client: aws_sdk_s3::Client, config: Config) -> Self {
        Self {
            upload_repository: Box::new(UploadRepository { pool }),
        }
    }
}
