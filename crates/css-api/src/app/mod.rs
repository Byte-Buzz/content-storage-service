use aws_sdk_s3::Client;
use css_core::repository::Repositories;

use crate::{app::service::Services, config::Config};

pub mod service;

pub struct App {
    pub config: Config,

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
            config: config.clone(),

            services: Services::new(repository, config),
        }
    }
}
