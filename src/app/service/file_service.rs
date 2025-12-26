use futures_util::{StreamExt, TryStreamExt};
use serde_json::Value;
use tokio_util::io::StreamReader;

use crate::domain::{errors::ServiceError, interfaces, models};

pub struct FileService {
    app_repository: Box<dyn interfaces::AppInterface>,
    s3_repository: Box<dyn interfaces::S3Interface>,
    file_repository: Box<dyn interfaces::FileInterface>,
}

impl FileService {
    pub fn new(
        app_repository: Box<dyn interfaces::AppInterface>,
        s3_repository: Box<dyn interfaces::S3Interface>,
        file_repository: Box<dyn interfaces::FileInterface>,
    ) -> Self {
        Self {
            app_repository,
            s3_repository,
            file_repository,
        }
    }

    pub async fn upload_file(
        &self,
        file: models::UploadFile,
    ) -> Result<models::FileInfo, ServiceError> {
        let app = self.app_repository.get_by_id(file.upload.app_id).await?;

        let name = file
            .file
            .content_disposition()
            .and_then(|content| content.get_filename());

        if name.is_none() {
            return Err(ServiceError::BadRequest("invalid file name".to_string()));
        }
        let name = name.unwrap();

        let available_types = get_available_types(&file.upload.info);

        let file_ext = check_file_type(name, &available_types)?;

        let max_size =
            get_max_file_size(&file.upload.info).min(self.s3_repository.max_file_size().await);

        let mut current_size = 0;

        let stream = file
            .file
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
            .map(move |v| {
                if let Ok(v) = &v {
                    current_size += v.len();
                    if current_size > max_size as usize {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "file size limit exceeded",
                        ));
                    }
                }

                v
            });

        let file_name = format!("{}.{}", file.upload.id, file_ext);
        let key = format!("{}/{}", app.name, file_name);

        let mut stream = Box::new(StreamReader::new(stream));

        let e_tag = self
            .s3_repository
            .upload_file(&mut stream, key.clone())
            .await?;

        // TODO: add file to db and remove from upload table

        Ok(models::FileInfo {
            key,
            filename: file_name,
        })
    }
}

impl Clone for FileService {
    fn clone(&self) -> Self {
        Self {
            app_repository: self.app_repository.clone_box(),
            s3_repository: self.s3_repository.clone_box(),
            file_repository: self.file_repository.clone_box(),
        }
    }
}

fn get_max_file_size(info: &Value) -> u64 {
    match info.get("max_size") {
        Some(Value::Number(v)) => v.as_u64().unwrap_or(u64::MAX),
        _ => u64::MAX,
    }
}

fn check_file_type(name: &str, available_types: &Vec<&str>) -> Result<String, ServiceError> {
    let guessed_mimes = mime_guess::from_path(name).iter().collect::<Vec<_>>();

    if guessed_mimes.is_empty() {
        return Err(ServiceError::BadRequest("invalid file type".to_string()));
    }

    let path = std::path::Path::new(name);
    let file_ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    for allowed in available_types {
        if *allowed == "*/*" {
            return Ok(file_ext.clone());
        }

        if allowed.contains('/') {
            for mime in &guessed_mimes {
                if mime_matches(allowed, mime) {
                    let ext = mime_guess::get_mime_extensions(mime)
                        .and_then(|exts| exts.first())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| file_ext.clone());
                    return Ok(ext);
                }
            }
        } else if allowed.starts_with('.') && name.ends_with(allowed) {
            return Ok(name.to_string().split_off(name.len() - allowed.len()));
        }
    }

    Err(ServiceError::BadRequest(format!(
        "File type {:?} is not allowed",
        guessed_mimes
    )))
}

fn get_available_types(info: &Value) -> Vec<&str> {
    match info.get("available_types") {
        Some(Value::Array(v)) => v.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>(),
        Some(Value::String(mime)) => vec![mime.as_str()],
        _ => vec!["*/*"],
    }
}

fn mime_matches(allowed: &str, actual: &mime_guess::Mime) -> bool {
    if allowed == "*/*" {
        return true;
    }

    if let Some((a_type, a_sub)) = allowed.split_once('/') {
        if a_sub == "*" {
            return actual.type_().as_str() == a_type;
        }
    }

    actual.as_ref() == allowed
}
