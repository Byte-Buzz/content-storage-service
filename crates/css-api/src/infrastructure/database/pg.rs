use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::infrastructure::config::DatabaseConfig;

pub async fn create_pg_pool(config: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.max_open_conns)
        .min_connections(config.max_idle_conns)
        .max_lifetime(Duration::from_secs(config.conn_max_lifetime))
        .connect(&config.url)
        .await
}
