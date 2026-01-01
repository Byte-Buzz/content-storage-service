use crate::{
    app::repository::map_sqlx_error,
    domain::{errors::RepositoryError, interfaces::FileInterface, models},
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
    async fn create_file_and_delete_upload(
        &self,
        file: models::CreateFile,
    ) -> Result<models::File, crate::domain::errors::RepositoryError> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;

        let result1 = sqlx::query_as!(
            models::File,
            r#"
                INSERT INTO files (id, app_id, filename, content_type,
                    e_tag, access, miniatures, miniature_extension)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING id, app_id, filename, content_type, e_tag,
                    access as "access!: _", miniatures, miniature_extension,
                    created_at
            "#,
            file.id,
            file.app_id,
            file.filename,
            file.content_type,
            file.e_tag,
            file.access as _,
            &file.miniatures,
            file.miniature_extension
        )
        .fetch_one(&mut *tx)
        .await;

        let result1 = match result1 {
            Ok(v) => v,
            Err(e) => {
                tx.rollback().await.map_err(map_sqlx_error)?;
                return Err(map_sqlx_error(e));
            }
        };

        let result2 = sqlx::query!(
            r#"
                DELETE FROM uploads WHERE id = $1
            "#,
            file.id
        )
        .execute(&mut *tx)
        .await;

        if let Err(e) = result2 {
            tx.rollback().await.map_err(map_sqlx_error)?;
            return Err(map_sqlx_error(e));
        }

        tx.commit().await.map_err(map_sqlx_error)?;

        Ok(result1)
    }

    async fn create_temp_file_and_delete_upload(
        &self,
        file: models::CreateTempFile,
    ) -> Result<models::TempFile, crate::domain::errors::RepositoryError> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;

        let result1 = sqlx::query_as!(
            models::TempFile,
            r#"
                INSERT INTO temp_files (id, app_id, filename, content_type, expires_at)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id, app_id, filename, created_at, content_type, expires_at
            "#,
            file.id,
            file.app_id,
            file.filename,
            file.content_type,
            file.expires_at
        )
        .fetch_one(&mut *tx)
        .await;

        let result1 = match result1 {
            Ok(v) => v,
            Err(e) => {
                tx.rollback().await.map_err(map_sqlx_error)?;
                return Err(map_sqlx_error(e));
            }
        };

        let result2 = sqlx::query!(
            r#"
                DELETE FROM uploads WHERE id = $1
            "#,
            file.id
        )
        .execute(&mut *tx)
        .await;

        if let Err(e) = result2 {
            tx.rollback().await.map_err(map_sqlx_error)?;
            return Err(map_sqlx_error(e));
        }

        tx.commit().await.map_err(map_sqlx_error)?;

        Ok(result1)
    }

    async fn get_by_id(&self, id: uuid::Uuid) -> Result<models::File, RepositoryError> {
        sqlx::query_as!(
            models::File,
            r#"
                SELECT id, app_id, filename, content_type, e_tag,
                    access as "access!: _", miniatures, miniature_extension,
                    created_at
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
