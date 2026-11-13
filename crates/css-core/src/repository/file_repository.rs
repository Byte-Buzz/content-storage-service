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

    async fn get_by_ids_with_app(
        &self,
        ids: Vec<uuid::Uuid>,
    ) -> Result<Vec<models::FileForDownload>, RepositoryError> {
        sqlx::query_as!(
            models::FileForDownload,
            r#"
                SELECT f.id, f.app_id, a.name AS app_name,
                    f.content_type, f.variants
                FROM files f JOIN app a ON a.id = f.app_id
                WHERE f.id = ANY($1)
            "#,
            &ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)
    }

    fn clone_box(&self) -> Box<dyn FileInterface + Send + Sync> {
        Box::new(FileRepository {
            pool: self.pool.clone(),
        })
    }
}
