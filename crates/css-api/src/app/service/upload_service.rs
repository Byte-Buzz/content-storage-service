use css_core::{
    domain::{interfaces, models, storage_keys},
    repository::Repositories,
};

use crate::{app::service::error::ServiceError, config::UploadConfig};

pub struct UploadService {
    upload_repository: Box<dyn interfaces::UploadInterface>,
    app_repository: Box<dyn interfaces::AppInterface>,
    s3_repository: Box<dyn interfaces::S3Interface>,
    upload_config: UploadConfig,
}

impl UploadService {
    pub fn new(repositories: Repositories, upload_config: UploadConfig) -> Self {
        Self {
            upload_repository: repositories.upload_repository,
            app_repository: repositories.app_repository,
            s3_repository: repositories.s3_repository,
            upload_config,
        }
    }

    pub async fn create(
        &self,
        upload: models::UploadRequest,
    ) -> Result<models::UploadResponse, ServiceError> {
        let name = &upload.app_name;
        let valid = !name.is_empty()
            && name.len() <= 63
            && name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_');
        if !valid {
            return Err(ServiceError::InvalidArgument(
                "app_name must be a valid ascii string".into(),
            ));
        }

        let max_size = match i64::try_from(upload.max_size) {
            Ok(v) if v != 0 => v,
            _ => {
                return Err(ServiceError::InvalidArgument(
                    "max_size must be a positive integer".into(),
                ));
            }
        };

        if upload.max_duration_seconds == Some(0) {
            return Err(ServiceError::InvalidArgument(
                "max_duration_seconds needs to be greater than 0".into(),
            ));
        }

        let max_duration_seconds = upload
            .max_duration_seconds
            .map(i32::try_from)
            .transpose()
            .map_err(|_| {
                ServiceError::InvalidArgument("max_duration_seconds is too large".into())
            })?;

        let ttl = match upload.upload_url_ttl {
            Some(v) if v >= self.upload_config.min_ttl && v <= self.upload_config.max_ttl => v,
            None => self.upload_config.default_ttl,
            _ => {
                return Err(ServiceError::InvalidArgument(format!(
                    "upload_url_ttl must be between {} and {} seconds",
                    self.upload_config.min_ttl.as_secs(),
                    self.upload_config.max_ttl.as_secs(),
                )));
            }
        };

        let (kind, subtype) = match upload.content_type.split_once("/") {
            Some(v) => v,
            _ => {
                return Err(ServiceError::InvalidArgument(format!(
                    "content_type must be a valid mime type"
                )));
            }
        };

        if kind.contains("/")
            || subtype.contains("/")
            || kind.trim().is_empty()
            || subtype.trim().is_empty()
        {
            return Err(ServiceError::InvalidArgument(format!(
                "content_type must be a valid mime type"
            )));
        }

        if upload
            .info
            .as_ref()
            .is_some_and(|i| i.len() > self.upload_config.max_info_size)
        {
            return Err(ServiceError::InvalidArgument(format!(
                "info must be less than or equal to {} bytes",
                self.upload_config.max_info_size
            )));
        }

        let app = self
            .app_repository
            .create_if_not_exists(&upload.app_name)
            .await?;

        let id = uuid::Uuid::new_v4();
        let now = chrono::Utc::now();

        let expires_at = now + ttl;

        let stored = self
            .upload_repository
            .create(models::CreateUpload {
                id,
                app_id: app.id,
                content_type: upload.content_type,
                accepted_content_types: upload.accepted_content_types,
                max_size,
                max_duration_seconds,
                tasks: upload.tasks,
                external_tasks: upload.external_tasks,
                info: upload.info,
                expires_at,
            })
            .await?;

        let presigned_post =
            self.s3_repository
                .create_presigned_post(models::PresignedPostRequest {
                    key: storage_keys::temp_key(&app.name, id),
                    content_type: stored.content_type,
                    max_size: max_size as u64,
                    expires_at,
                })?;

        Ok(models::UploadResponse {
            file_id: id,
            upload_url: presigned_post.url,
            form_fields: presigned_post.fields,
            expires_at: presigned_post.expires_at,
        })
    }
}

impl Clone for UploadService {
    fn clone(&self) -> Self {
        Self {
            upload_repository: self.upload_repository.clone_box(),
            app_repository: self.app_repository.clone_box(),
            s3_repository: self.s3_repository.clone_box(),
            upload_config: self.upload_config.clone(),
        }
    }
}
