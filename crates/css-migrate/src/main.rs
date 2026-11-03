use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::EnvFilter;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let url = std::env::var("CSS_DATABASE_URL").map_err(|_| "CSS_DATABASE_URL is not set")?;

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&url)
        .await?;

    tracing::info!(available = MIGRATOR.iter().count(), "running migrations");
    MIGRATOR.run(&pool).await?;
    tracing::info!("migrations are up to date");

    pool.close().await;
    Ok(())
}
