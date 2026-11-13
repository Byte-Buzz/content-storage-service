use std::fmt::Debug;

use super::env::get_env_or;

/// Database connection settings.
///
/// - CSS_DATABASE_URL: PostgreSQL URL. Required.
/// - CSS_DB_MAX_OPEN_CONNS: max open connections. Defaults to 10.
/// - CSS_DB_MAX_IDLE_CONNS: min idle connections. Defaults to 5.
/// - CSS_DB_CONN_MAX_LIFETIME: connection lifetime in seconds. Defaults to 3600.
#[derive(Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_open_conns: u32,
    pub max_idle_conns: u32,
    pub conn_max_lifetime: u64,
}

impl Debug for DatabaseConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DatabaseConfig")
            .field("url", &"...")
            .field("max_open_conns", &self.max_open_conns)
            .field("max_idle_conns", &self.max_idle_conns)
            .field("conn_max_lifetime", &self.conn_max_lifetime)
            .finish()
    }
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Self {
            url: get_env_or("DATABASE_URL", ""),
            max_open_conns: get_env_or("DB_MAX_OPEN_CONNS", "10").parse()?,
            max_idle_conns: get_env_or("DB_MAX_IDLE_CONNS", "5").parse()?,
            conn_max_lifetime: get_env_or("DB_CONN_MAX_LIFETIME", "3600").parse()?,
        };
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), String> {
        if self.url.trim().is_empty() {
            return Err("CSS_DATABASE_URL cannot be empty".to_string());
        }
        Ok(())
    }
}
