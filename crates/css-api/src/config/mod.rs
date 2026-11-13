mod server;

use css_core::infrastructure::config::{DatabaseConfig, S3Config};
pub use server::{GrpcConfig, ServerConfig};

/// Main configuration structure holding all sub-configurations.
#[derive(Debug, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub grpc: GrpcConfig,
    pub s3: S3Config,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            database: DatabaseConfig::from_env()?,
            server: ServerConfig::from_env()?,
            grpc: GrpcConfig::from_env()?,
            s3: S3Config::from_env()?,
        })
    }
}
