use crate::infrastructure::config::{Config, DatabaseConfig, S3Config, ServerConfig};

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
pub fn parse_env_config() -> Result<Config, Box<dyn std::error::Error>> {
    Ok(Config {
        database: DatabaseConfig {
            url: get_env("DATABASE_URL").unwrap_or_else(|_| "".to_string()),
            max_open_conns: get_env("DB_MAX_OPEN_CONNS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
            max_idle_conns: get_env("DB_MAX_IDLE_CONNS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()?,
            conn_max_lifetime: get_env("DB_CONN_MAX_LIFETIME")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()?,
        },
        server: ServerConfig {
            host: get_env("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: get_env("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
        },
        s3: S3Config {
            endpoint: get_env("S3_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:9000".to_string()),
            access_key: get_env("S3_ACCESS_KEY").unwrap_or_else(|_| "rustfsadmin".to_string()),
            secret_key: get_env("S3_SECRET_KEY").unwrap_or(String::new()),
        },
    })
}

/// Get an environment variable.
///
/// This function takes a key and returns the value of the environment
/// variable with the prefix "CSS_" and the given key. If the
/// environment variable does not exist, it returns an error.
pub fn get_env(key: &str) -> Result<String, std::env::VarError> {
    let full_key = format!("CSS_{}", key);
    std::env::var(&full_key)
}
