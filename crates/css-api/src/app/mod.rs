use aws_sdk_s3::Client;
use css_core::repository::Repositories;

use crate::{app::service::Services, config::Config};

pub mod service;

pub struct App {
    pub s3_client: Client,
    pub pg_pool: sqlx::PgPool,
    pub config: Config,

    pub repositories: Repositories,
    pub services: Services,
}

impl App {
    pub fn new(
        s3_client: Client,
        presign_s3_client: Client,
        pg_pool: sqlx::PgPool,
        config: Config,
    ) -> Self {
        let repository = Repositories::new(
            pg_pool.clone(),
            s3_client.clone(),
            presign_s3_client,
            &config.s3,
        );

        Self {
            s3_client,
            pg_pool,
            config: config.clone(),

            repositories: repository.clone(),
            services: Services::new(repository, config),
        }
    }
}
