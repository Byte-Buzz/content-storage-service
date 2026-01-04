mod env;
mod validate;

/// Main configuration structure holding all sub-configurations.
#[derive(Debug, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub grpc: GrpcConfig,
    pub s3: S3Config,
}

/// Configuration structure for the web server.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub base_url: String,
}

/// Configuration structure for the gRPC server.
#[derive(Debug, Clone)]
pub struct GrpcConfig {
    pub host: String,
    pub port: u16,
}

/// Configuration structure for the database connection.
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_open_conns: u32,
    pub max_idle_conns: u32,
    pub conn_max_lifetime: u64,
}

/// Configuration structure for S3 storage.
#[derive(Debug, Clone)]
pub struct S3Config {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub temp_bucket: String,
    pub bucket: String,
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
    /// - CSS_S3_BUCKET: The name of the final bucket. Defaults to "css".
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = env::parse_env_config()?;
        validate::validate_config(&config)?;
        Ok(config)
    }

    /// Returns the address of the web server in the format "host:port".
    pub fn get_http_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    /// Returns the address of the gRPC server in the format "host:port".
    pub fn get_grpc_address(&self) -> String {
        format!("{}:{}", self.grpc.host, self.grpc.port)
    }
}
