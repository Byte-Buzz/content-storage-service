use chrono::Days;

use crate::{
    app::repository::map_sqlx_error,
    domain::{
        errors::RepositoryError,
        interfaces::UploadInterface,
        models::{CreateUpload, Upload},
    },
};

pub struct UploadRepository {
    pub pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl UploadInterface for UploadRepository {
    async fn create(&self, upload: CreateUpload) -> Result<u64, RepositoryError> {
        sqlx::query!(
            r#"
                INSERT INTO uploads (id, app_id, settings, info, secret, expires_at)
                VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            upload.id,
            upload.app_id,
            upload.settings,
            upload.info,
            upload.secret,
            upload
                .expires_at
                .unwrap_or(chrono::Utc::now().naive_utc() + Days::new(1)),
        )
        .execute(&self.pool)
        .await
        .map(|v| v.rows_affected())
        .map_err(map_sqlx_error)
    }

    async fn delete(&self, id: uuid::Uuid) -> Result<u64, RepositoryError> {
        sqlx::query!(
            r#"
                DELETE FROM uploads WHERE id = $1
            "#,
            id,
        )
        .execute(&self.pool)
        .await
        .map(|v| v.rows_affected())
        .map_err(map_sqlx_error)
    }

    async fn get_by_id(&self, id: uuid::Uuid) -> Result<Upload, RepositoryError> {
        sqlx::query_as!(
            Upload,
            r#"
                SELECT id, app_id, settings, info, secret, created_at, expires_at
                FROM uploads
                WHERE id = $1
            "#,
            id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_sqlx_error)
    }

    fn clone_box(&self) -> Box<dyn UploadInterface + Send + Sync> {
        Box::new(Self {
            pool: self.pool.clone(),
        })
    }
}
