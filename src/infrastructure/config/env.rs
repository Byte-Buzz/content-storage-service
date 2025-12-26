use crate::infrastructure::config::{Config, DatabaseConfig, GrpcConfig, S3Config, ServerConfig};

/// Parse configuration from environment variables.
///
/// This function reads configuration values from environment variables and
/// returns a Config instance. If any environment variable is not set,
/// it will use a default value. If the default value is not provided,
/// it will return an error.
///
/// The environment variables are as follows:
///
/// - CSS_DATABASE_URL: The URL of the PostgreSQL database.
///   Defaults to an empty string.
/// - CSS_DB_MAX_OPEN_CONNS: The maximum number of open connections to the
///   database. Defaults to 10.
/// - CSS_DB_MAX_IDLE_CONNS: The maximum number of idle connections to the
///   database. Defaults to 5.
/// - CSS_DB_CONN_MAX_LIFETIME: The maximum lifetime of a connection in
///   seconds. Defaults to 3600 (1 hour).
/// - CSS_SERVER_HOST: The host of the web server. Defaults to "0.0.0.0".
/// - CSS_SERVER_PORT: The port of the web server. Defaults to 8080.
/// - CSS_BASE_URL: The base URL of the web server. Defaults to "".
/// - CSS_GRPC_HOST: The host of the gRPC server. Defaults to "[::1]".
/// - CSS_GRPC_PORT: The port of the gRPC server. Defaults to 50051.
/// - CSS_S3_ENDPOINT: The URL of the S3 server. Defaults to "http://localhost:9000".
/// - CSS_S3_ACCESS_KEY: The access key of the S3 server. Defaults to "rustfsadmin".
/// - CSS_S3_SECRET_KEY: The secret key of the S3 server. Defaults to an empty string.
/// - CSS_S3_TEMP_BUCKET: The name of the temporary bucket. Defaults to "css-temp".
/// - CSS_S3_BUCKET: The name of the bucket. Defaults to "css".
pub fn parse_env_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut base_url = get_env("SERVER_BASE_URL").unwrap_or_else(|_| "".to_string());
    if !base_url.ends_with('/') {
        base_url.push('/');
    }

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
            base_url,
        },
        grpc: GrpcConfig {
            host: get_env("GRPC_HOST").unwrap_or_else(|_| "[::1]".to_string()),
            port: get_env("GRPC_PORT")
                .unwrap_or_else(|_| "50051".to_string())
                .parse()?,
        },
        s3: S3Config {
            endpoint: get_env("S3_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:9000".to_string()),
            access_key: get_env("S3_ACCESS_KEY").unwrap_or_else(|_| "rustfsadmin".to_string()),
            secret_key: get_env("S3_SECRET_KEY").unwrap_or(String::new()),
            temp_bucket: get_env("S3_TEMP_BUCKET").unwrap_or_else(|_| "css-temp".to_string()),
            bucket: get_env("S3_BUCKET").unwrap_or_else(|_| "css".to_string()),
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
