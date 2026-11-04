use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use css_core::{
    domain::{interfaces, models},
    repository::Repositories,
};
use rand::{TryRngCore, rngs::OsRng};
use sha2::{Digest, Sha256};

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
        let app = self
            .app_repository
            .create_if_not_exists(&upload.app_name)
            .await?;

        let mut bytes = [0u8; 32];

        OsRng.try_fill_bytes(&mut bytes).map_err(|e| {
            ServiceError::Internal(format!("Failed to generate upload secret: {}", e))
        })?;
        let secret = BASE64_URL_SAFE_NO_PAD.encode(&bytes);

        let hash = Sha256::digest(secret.as_bytes());

        let upload = models::CreateUpload {
            id: uuid::Uuid::new_v4(),
            app_id: app.id,
            filename: upload.filename,
            content_type: upload.content_type,
            max_size: upload.max_size as i32,
            settings: upload.settings,
            info: upload.info,
            access: upload.access,
            secret: hex::encode(hash),
            expires_at: upload.upload_url_ttl.map(|ttl| {
                let now = chrono::Utc::now().naive_utc();
                now + chrono::Duration::seconds(ttl as i64)
            }),
        };

        let mut upload = self.upload_repository.create(upload).await?;

        upload.secret = secret;

        Ok(upload)
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
