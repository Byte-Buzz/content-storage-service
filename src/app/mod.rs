use crate::{app::repository::Repositories, domain::interfaces, infrastructure::config::Config};

pub mod repository;

pub struct App {
    pub s3_client: aws_sdk_s3::Client,
    pub pg_pool: sqlx::PgPool,
    pub config: Config,
    pub repository: Repositories,
}

impl App {
    pub fn new(s3_client: aws_sdk_s3::Client, pg_pool: sqlx::PgPool, config: Config) -> Self {
        let repository = Repositories::new(pg_pool.clone(), s3_client.clone(), config.clone());

        Self {
            s3_client,
            pg_pool,
            config,

            repository,
        }
    }
}
