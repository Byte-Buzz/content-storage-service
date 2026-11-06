use crate::{
    domain::{errors::RepositoryError, interfaces::FileInterface, models},
    repository::map_sqlx_error,
};

pub struct FileRepository {
    pool: sqlx::PgPool,
}

impl FileRepository {
    #[inline(always)]
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl FileInterface for FileRepository {
    async fn get_by_id(&self, id: uuid::Uuid) -> Result<models::File, RepositoryError> {
        sqlx::query_as!(
            models::File,
            r#"
                SELECT id, app_id, content_type, variants,
                    visibility as "visibility!: _",
                    visibility_version, created_at
                FROM files
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_sqlx_error)
    }

    fn clone_box(&self) -> Box<dyn FileInterface + Send + Sync> {
        Box::new(FileRepository {
            pool: self.pool.clone(),
        })
    }
}
