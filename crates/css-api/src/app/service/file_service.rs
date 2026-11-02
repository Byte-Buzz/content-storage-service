use futures_util::{StreamExt, TryStreamExt};
use tokio_util::io::StreamReader;

use crate::{
    app::repository::Repositories,
    domain::{
        errors::{RepositoryError, ServiceError},
        interfaces,
        models::{self, FileResponse, FileResponseFile, UploadFileRequest},
    },
};

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

    pub async fn upload_file(
        &self,
        file: models::UploadFile,
    ) -> Result<models::FileInfo, ServiceError> {
        let app = self.app_repository.get_by_id(file.upload.app_id).await?;

        let max_size = (file.upload.max_size as u32).min(self.s3_repository.max_file_size().await);

        let mut current_size = 0;

        let stream = file
            .file
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
            .map(move |v| {
                if let Ok(v) = &v {
                    tracing::debug!("read {} bytes", v.len());
                    current_size += v.len();
                    if current_size > max_size as usize {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::FileTooLarge,
                            "file size limit exceeded",
                        ));
                    }
                }

                v
            });

        let file_name = file.upload.filename;
        let key = format!("{}/{}", app.name, file.upload.id);

        let file_settings = models::FileSettings::from(file.upload.settings);

        let mut stream = StreamReader::new(stream);

        if file_settings.need_temporary_storage() {
            self.s3_repository
                .upload_temp_file(UploadFileRequest {
                    id: file.upload.id,
                    key: key.clone(),
                    file: &mut stream,
                    content_type: file.upload.content_type.clone(),
                })
                .await
                .map_err(|e| {
                    tracing::error!("Failed to upload temp file: {}", e);
                    e
                })?;

            tracing::debug!("temp file uploaded");

            self.file_repository
                .create_temp_file_and_delete_upload(models::CreateTempFile {
                    id: file.upload.id,
                    app_id: file.upload.app_id,
                    filename: file_name.clone(),
                    content_type: file.upload.content_type,
                    expires_at: None,
                })
                .await
                .map_err(|e| {
                    tracing::error!("Failed to create temp file: {}", e);
                    e
                })?;
        } else {
            let res = self
                .s3_repository
                .upload_file(UploadFileRequest {
                    id: file.upload.id,
                    key: key.clone(),
                    file: &mut stream,
                    content_type: file.upload.content_type.clone(),
                })
                .await
                .map_err(|e| {
                    tracing::error!("Failed to upload file: {}", e);
                    e
                })?;

            tracing::debug!("file uploaded");

            self.file_repository
                .create_file_and_delete_upload(models::CreateFile {
                    id: file.upload.id,
                    app_id: file.upload.app_id,
                    filename: file_name.clone(),
                    content_type: file.upload.content_type,
                    access: Some(file.upload.access),
                    e_tag: res.clone().unwrap_or(file_name.clone()),
                    ..Default::default()
                })
                .await
                .map_err(|e| {
                    tracing::error!("Failed to create file: {}", e);
                    e
                })?;
        };

        Ok(models::FileInfo {
            key,
            filename: file_name,
        })
    }

    pub async fn get_file(
        &self,
        app: &str,
        file_id: uuid::Uuid,
        e_tag: Option<&str>,
        _query: models::PresignedQuery,
    ) -> Result<FileResponse, ServiceError> {
        tracing::info!("get file");
        tracing::debug!("get file: {}/{}", app, file_id);

        let app = match app.parse::<u32>() {
            Ok(v) => self.app_repository.get_by_id(v as i64).await?,
            Err(_) => self.app_repository.get_by_name(app).await?,
        };

        tracing::debug!("app: {:#?}", app);

        let file = self.file_repository.get_by_id(file_id).await?;

        tracing::debug!("file: {:#?}", file);

        if file.app_id != app.id {
            tracing::debug!("file does not belong to app");
            return Err(RepositoryError::NotFound.into());
        }

        if let Some(e_tag) = e_tag {
            if e_tag != file.e_tag {
                tracing::debug!("e_tag does not match");
                return Ok(FileResponse::NotModified);
            }
        }

        let key = format!("{}/{}", app.name, file_id);

        tracing::debug!("key: {}", key);

        let mut file_response = self.s3_repository.get_file(&key).await?;

        if file_response.e_tag.is_empty() {
            file_response.e_tag = file.e_tag;
        }

        if file_response.content_type.is_empty() {
            file_response.content_type = file.content_type;
        }

        Ok(FileResponseFile {
            file: Box::new(file_response.file),
            content_type: file_response.content_type,
            e_tag: file_response.e_tag,
            filename: file.filename,
            size: file_response.size,
        }
        .into())
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
