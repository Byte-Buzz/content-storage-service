use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use rand::{RngCore, TryRngCore, rngs::OsRng};
use sha2::{Digest, Sha256};

use crate::domain::{
    errors::{self, ServiceError},
    interfaces, models,
};

pub struct UploadService {
    upload_repository: Box<dyn interfaces::UploadInterface>,
    app_repository: Box<dyn interfaces::AppInterface>,
}

impl UploadService {
    pub fn new(
        upload_repository: Box<dyn interfaces::UploadInterface>,
        app_repository: Box<dyn interfaces::AppInterface>,
    ) -> Self {
        Self {
            upload_repository,
            app_repository,
        }
    }

    pub async fn create(
        &self,
        upload: models::CreateNewUpload,
    ) -> Result<models::Upload, errors::ServiceError> {
        let app = self
            .app_repository
            .create_if_not_exists(&upload.app_name)
            .await?;

        let mut bytes = [0u8; 32];

        OsRng.try_fill_bytes(&mut bytes).map_err(|e| {
            errors::ServiceError::Internal(format!("Failed to generate upload secret: {}", e))
        })?;
        let secret = BASE64_URL_SAFE_NO_PAD.encode(&bytes);

        let hash = Sha256::digest(secret.as_bytes());

        let upload = models::CreateUpload {
            id: uuid::Uuid::new_v4(),
            app_id: app.id,
            settings: upload.settings,
            info: upload.info,
            secret: hex::encode(hash),
            expires_at: upload.expires_at,
        };

        let mut upload = self.upload_repository.create(upload).await?;

        upload.secret = secret;

        Ok(upload)
    }

    pub async fn check_upload(
        &self,
        id: uuid::Uuid,
        secret: &str,
    ) -> Result<models::Upload, errors::ServiceError> {
        tracing::info!("checking upload");
        let upload = self.upload_repository.get_by_id(id).await?;

        tracing::debug!("upload: {:#?}", upload);

        if upload.expires_at < chrono::Utc::now().naive_utc() {
            return Err(ServiceError::Forbidden);
        }

        let hash = Sha256::digest(secret.as_bytes());
        let provided_hash = hex::encode(hash);

        if subtle::ConstantTimeEq::ct_eq(upload.secret.as_bytes(), provided_hash.as_bytes()).into()
        {
            tracing::info!("upload secret matches");
            Ok(upload)
        } else {
            tracing::info!("upload secret does not match");
            Err(ServiceError::Forbidden)
        }
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
