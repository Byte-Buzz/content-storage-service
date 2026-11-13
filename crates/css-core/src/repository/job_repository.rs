use crate::{
    domain::{errors::RepositoryError, interfaces::JobInterface, models::ProcessingJobKind},
    repository::map_sqlx_error,
};

pub struct JobRepository {
    pool: sqlx::PgPool,
}

impl JobRepository {
    #[inline(always)]
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl JobInterface for JobRepository {
    async fn enqueue(
        &self,
        file_id: uuid::Uuid,
        kind: ProcessingJobKind,
    ) -> Result<bool, RepositoryError> {
        sqlx::query!(
            r#"
                INSERT INTO processing_jobs (file_id, kind)
                    VALUES ($1, $2)
                ON CONFLICT (file_id, kind) DO NOTHING
            "#,
            file_id,
            kind as _
        )
        .execute(&self.pool)
        .await
        .map(|v| v.rows_affected() != 0)
        .map_err(map_sqlx_error)
    }

    fn clone_box(&self) -> Box<dyn JobInterface + Send + Sync> {
        Box::new(JobRepository {
            pool: self.pool.clone(),
        })
    }
}
