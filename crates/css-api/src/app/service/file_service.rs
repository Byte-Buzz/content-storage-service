use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use css_core::{
    domain::{
        interfaces,
        models::{DownloadUrlRequest, PresignedGet, PresignedGetRequest},
        storage_keys::{ORIGINAL_VARIANT, variant_key},
    },
    repository::Repositories,
};

use crate::{app::service::error::ServiceError, config::DownloadConfig};

pub struct FileService {
    s3_repository: Box<dyn interfaces::S3Interface>,
    file_repository: Box<dyn interfaces::FileInterface>,
    config: DownloadConfig,
}

impl FileService {
    pub fn new(repositories: Repositories, config: DownloadConfig) -> Self {
        Self {
            s3_repository: repositories.s3_repository,
            file_repository: repositories.file_repository,
            config,
        }
    }

    pub async fn get_download_urls(
        &self,
        items: Vec<DownloadUrlRequest>,
    ) -> Result<Vec<PresignedGet>, ServiceError> {
        if items.len() > self.config.max_items as usize {
            return Err(ServiceError::InvalidArgument("too many items".into()));
        }

        let window = self.config.url_window.as_secs() as i64;
        let now = Utc::now().timestamp();
        let signed_at = DateTime::from_timestamp(now - now % window, 0).unwrap();
        let expires_in = self.config.url_window * 2;
        let expires_at = signed_at + expires_in;

        let ids: Vec<uuid::Uuid> = items
            .iter()
            .map(|i| i.file_id)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        let files = self
            .file_repository
            .get_by_ids_with_app(ids)
            .await?
            .into_iter()
            .map(|v| (v.id, v))
            .collect::<HashMap<_, _>>();

        let mut urls = Vec::with_capacity(items.len());

        for item in items {
            let Some(file) = files.get(&item.file_id) else {
                continue;
            };

            let variant = item.variant.as_deref().unwrap_or(ORIGINAL_VARIANT);
            if variant != ORIGINAL_VARIANT && !file.variants.iter().any(|v| v == variant) {
                continue;
            }

            let download_name = item
                .download_name
                .filter(|n| variant == ORIGINAL_VARIANT && !n.trim().is_empty())
                .map(|n| truncate_name(&n, self.config.max_name_length as usize));

            let url = self
                .s3_repository
                .presign_get(PresignedGetRequest {
                    key: variant_key(&file.app_name, variant, file.id),
                    download_name,
                    signed_at,
                    expires_in,
                })
                .await?;

            urls.push(PresignedGet {
                file_id: file.id,
                url,
                variant: item.variant,
                expires_at,
            });
        }

        Ok(urls)
    }
}

impl Clone for FileService {
    fn clone(&self) -> Self {
        Self {
            s3_repository: self.s3_repository.clone_box(),
            file_repository: self.file_repository.clone_box(),
            config: self.config.clone(),
        }
    }
}

fn truncate_name(name: &str, max: usize) -> String {
    if name.chars().count() <= max {
        return name.to_string();
    }
    match name.rsplit_once('.') {
        Some((stem, ext)) if ext.chars().count() < 10 => {
            let keep = max.saturating_sub(ext.chars().count() + 1);
            format!("{}.{ext}", stem.chars().take(keep).collect::<String>())
        }
        _ => name.chars().take(max).collect(),
    }
}
