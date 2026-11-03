use crate::infrastructure::config::env::get_env_or;

/// S3 configuration.
///
/// - CSS_S3_ENDPOINT: endpoint of the s3 server. Defaults to "http://localhost:9000".
/// - CSS_S3_ACCESS_KEY: access key of the s3 server. Defaults to "rustfsadmin".
/// - CSS_S3_SECRET_KEY: secret key of the s3 server. Required.
/// - CSS_S3_TEMP_BUCKET: temporary bucket name of the s3 server. Defaults to "css-temp".
/// - CSS_S3_BUCKET: bucket name of the s3 server. Defaults to "css".
#[derive(Debug, Clone)]
pub struct S3Config {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub temp_bucket: String,
    pub bucket: String,
}

impl S3Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Self {
            endpoint: get_env_or("S3_ENDPOINT", "http://localhost:9000"),
            access_key: get_env_or("S3_ACCESS_KEY", "rustfsadmin"),
            secret_key: get_env_or("S3_SECRET_KEY", ""),
            temp_bucket: get_env_or("S3_TEMP_BUCKET", "css-temp"),
            bucket: get_env_or("S3_BUCKET", "css"),
        };

        config.validate()?;

        Ok(config)
    }

    fn validate(&self) -> Result<(), String> {
        if self.endpoint.trim().is_empty() {
            return Err("CSS_S3_ENDPOINT cannot be empty".to_string());
        }

        if self.access_key.trim().is_empty() {
            return Err("CSS_S3_ACCESS_KEY is required".to_string());
        }

        if self.secret_key.trim().is_empty() {
            return Err("CSS_S3_SECRET_KEY is required".to_string());
        }

        if self.temp_bucket.trim().is_empty() {
            return Err("CSS_S3_TEMP_BUCKET cannot be empty".to_string());
        }

        if self.bucket.trim().is_empty() {
            return Err("CSS_S3_BUCKET cannot be empty".to_string());
        }

        Ok(())
    }
}
