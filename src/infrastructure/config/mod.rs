mod env;
mod validate;

/// Main configuration structure holding all sub-configurations.
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub s3: S3Config,
}

/// Configuration structure for the web server.
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

/// Configuration structure for the database connection.
pub struct DatabaseConfig {
    pub url: String,
    pub max_open_conns: u32,
    pub max_idle_conns: u32,
    pub conn_max_lifetime: u64,
}

/// Configuration structure for S3 storage.
pub struct S3Config {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
}

impl Config {
    /// Parse configuration from environment variables.
    ///
    /// This function reads configuration values from environment variables and
    /// returns a Config instance. If any environment variable is not set,
    /// it will use a default value. If the default value is not provided,
    /// it will return an error.
    ///
    /// The environment variables are as follows:
    ///
    /// - DATABASE_URL: The URL of the PostgreSQL database.
    ///   Defaults to an empty string.
    /// - DB_MAX_OPEN_CONNS: The maximum number of open connections to the
    ///   database. Defaults to 10.
    /// - DB_MAX_IDLE_CONNS: The maximum number of idle connections to the
    ///   database. Defaults to 5.
    /// - DB_CONN_MAX_LIFETIME: The maximum lifetime of a connection in
    ///   seconds. Defaults to 3600 (1 hour).
    /// - SERVER_HOST: The host of the web server. Defaults to "0.0.0.0".
    /// - SERVER_PORT: The port of the web server. Defaults to 8080.
    /// - S3_ENDPOINT: The URL of the S3 server. Defaults to "http://localhost:9000".
    /// - S3_ACCESS_KEY: The access key of the S3 server. Defaults to "rustfsadmin".
    /// - S3_SECRET_KEY: The secret key of the S3 server. Defaults to an empty string.
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = env::parse_env_config()?;
        validate::validate_config(&config)?;
        Ok(config)
    }
}
