use css_core::infrastructure::config::DatabaseConfig;
use sqlx::postgres::PgPoolOptions;

use crate::MIGRATOR;

pub async fn run_db(config: DatabaseConfig) -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&config.url)
        .await?;

    tracing::info!(available = MIGRATOR.iter().count(), "running migrations");
    MIGRATOR.run(&pool).await?;
    tracing::info!("migrations are up to date");

    pool.close().await;

    Ok(())
}
