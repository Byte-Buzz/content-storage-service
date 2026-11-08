use css_core::infrastructure::config::{DatabaseConfig, S3Config};
use tracing_subscriber::EnvFilter;

mod db;
mod storage;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let cmd = std::env::args().nth(1);
    let (db, storage) = match cmd {
        Some(cmd) => match cmd.as_str() {
            "db" => (Some(DatabaseConfig::from_env()?), None),
            "storage" => (None, Some(S3Config::from_env()?)),
            _ => return Err("usage: css-migrate [db|storage]".into()),
        },
        None => (
            Some(DatabaseConfig::from_env()?),
            Some(S3Config::from_env()?),
        ),
    };

    if let Some(config) = db {
        db::run_db(config).await?;
    }

    if let Some(config) = storage {
        storage::run_storage(config).await?;
    }

    Ok(())
}
