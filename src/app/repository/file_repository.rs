use crate::domain::{interfaces::FileInterface, models};

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
    async fn create(
        &self,
        file: models::File,
    ) -> Result<models::File, crate::domain::errors::RepositoryError> {
        Ok(file)
    }

    fn clone_box(&self) -> Box<dyn FileInterface + Send + Sync> {
        Box::new(FileRepository {
            pool: self.pool.clone(),
        })
    }
}
