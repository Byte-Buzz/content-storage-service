use css_core::{
    domain::{interfaces, models::ProcessingJobKind, storage_keys::parse_temp_key},
    repository::Repositories,
};

use crate::app::service::error::ServiceError;

pub struct StorageEventService {
    job_repository: Box<dyn interfaces::JobInterface>,
    temp_bucket: String,
}

impl StorageEventService {
    pub fn new(repositories: Repositories, temp_bucket: String) -> Self {
        Self {
            job_repository: repositories.job_repository,
            temp_bucket,
        }
    }

    pub async fn handle_object_created(&self, bucket: &str, key: &str) -> Result<(), ServiceError> {
        if bucket != self.temp_bucket {
            return Err(ServiceError::InvalidArgument(format!(
                "unexpected bucket: {bucket}"
            )));
        }

        let (_, id) = match parse_temp_key(key) {
            Some(v) => v,
            _ => return Err(ServiceError::InvalidArgument("invalid key".into())),
        };

        let inserted = self
            .job_repository
            .enqueue(id, ProcessingJobKind::Ingest)
            .await?;

        if !inserted {
            tracing::info!(%key, "duplicate storage event, job already exists");
        }

        Ok(())
    }
}

impl Clone for StorageEventService {
    fn clone(&self) -> Self {
        Self {
            job_repository: self.job_repository.clone_box(),
            temp_bucket: self.temp_bucket.clone(),
        }
    }
}
