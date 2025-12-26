use crate::{
    app::{repository::Repositories, service::Services},
    infrastructure::{config::Config, storage::S3Client},
};

pub mod repository;
pub mod service;

pub struct App {
    pub s3_client: S3Client,
    pub pg_pool: sqlx::PgPool,
    pub config: Config,

    pub repositories: Repositories,
    pub services: Services,
}

impl App {
    pub fn new(s3_client: S3Client, pg_pool: sqlx::PgPool, config: Config) -> Self {
        let repository = Repositories::new(pg_pool.clone(), s3_client.clone());

        Self {
            s3_client,
            pg_pool,
            config,

            repositories: repository.clone(),
            services: Services::new(repository),
        }
    }
}
