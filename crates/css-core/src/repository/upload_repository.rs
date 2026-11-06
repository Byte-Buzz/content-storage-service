use crate::{
    domain::{
        errors::RepositoryError,
        interfaces::UploadInterface,
        models::{self, CreateUpload, ProcessingTask, Upload},
    },
    repository::map_sqlx_error,
};

pub struct UploadRepository {
    pool: sqlx::PgPool,
}

impl UploadRepository {
    #[inline(always)]
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UploadInterface for UploadRepository {
    async fn create(&self, upload: CreateUpload) -> Result<models::Upload, RepositoryError> {
        sqlx::query_as!(
            models::Upload,
            r#"
                INSERT INTO uploads (id, app_id, content_type, accepted_content_types, max_size,
                    max_duration_seconds, tasks, external_tasks, info, expires_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                RETURNING id, app_id, content_type, accepted_content_types, max_size,
                    max_duration_seconds, tasks as "tasks!: _", external_tasks, info,
                    created_at, expires_at
            "#,
            upload.id,
            upload.app_id,
            upload.content_type,
            &upload.accepted_content_types,
            upload.max_size,
            upload.max_duration_seconds,
            &upload.tasks as &[ProcessingTask],
            &upload.external_tasks,
            upload.info,
            upload.expires_at,
        )
        .fetch_one(&self.pool)
        .await
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
                SELECT id, app_id, content_type, accepted_content_types, max_size,
                    max_duration_seconds, tasks as "tasks!: _", external_tasks,
                    info, created_at, expires_at
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
