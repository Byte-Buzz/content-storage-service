use aws_sdk_s3::Client;

use crate::{
    app::{repository::Repositories, service::Services},
    infrastructure::config::Config,
};

pub mod repository;
pub mod service;

pub struct App {
    pub s3_client: Client,
    pub pg_pool: sqlx::PgPool,
    pub config: Config,

    pub repositories: Repositories,
    pub services: Services,
}

impl App {
    pub fn new(s3_client: Client, pg_pool: sqlx::PgPool, config: Config) -> Self {
        let repository = Repositories::new(pg_pool.clone(), s3_client.clone(), &config.s3);

        Self {
            s3_client,
            pg_pool,
            config,

            repositories: repository.clone(),
            services: Services::new(repository),
        }
    }
}
