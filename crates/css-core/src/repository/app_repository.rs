use crate::{
    domain::{errors::RepositoryError, interfaces::AppInterface, models},
    repository::map_sqlx_error,
};

pub struct AppRepository {
    pool: sqlx::PgPool,
}

impl AppRepository {
    #[inline(always)]
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AppInterface for AppRepository {
    async fn create_if_not_exists(&self, name: &str) -> Result<models::App, RepositoryError> {
        sqlx::query_as!(
            models::App,
            r#"
                INSERT INTO app (name)
                VALUES ($1)
                ON CONFLICT (name) DO UPDATE
                    SET name = EXCLUDED.name
                RETURNING id, name
            "#,
            name,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_sqlx_error)
    }

    fn clone_box(&self) -> Box<dyn AppInterface + Send + Sync> {
        Box::new(AppRepository {
            pool: self.pool.clone(),
        })
    }
}
