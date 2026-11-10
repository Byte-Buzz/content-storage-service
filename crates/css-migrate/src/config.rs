use css_core::infrastructure::config::get_env_or;

/// Storage configuration
///
/// - CSS_S3_NOTIFY_ARN: ARN of the notification target for upload events. Subscription is skipped if empty
/// - CSS_S3_TEMP_TTL_DAYS: days to keep unprocessed uploads in the temp bucket.
///   Defaults to 6, long enough to survive a processing outage.
/// - CSS_S3_CORS_ORIGINS: comma-separated origins allowed to upload from the browser to the temp bucket. Defaults to "*"
pub struct StorageSetupConfig {
    pub notify_arn: String,
    pub temp_ttl_days: i32,
    pub cors_origins: Vec<String>,
}

impl StorageSetupConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Self {
            notify_arn: get_env_or("S3_NOTIFY_ARN", ""),
            temp_ttl_days: get_env_or("S3_TEMP_TTL_DAYS", "6").parse()?,
            cors_origins: get_env_or("S3_CORS_ORIGINS", "*")
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
        };

        config.validate()?;

        Ok(config)
    }

    fn validate(&self) -> Result<(), String> {
        if self.temp_ttl_days < 1 {
            return Err("CSS_S3_TEMP_TTL_DAYS must be at least 1".to_string());
        }

        if self.cors_origins.is_empty() {
            return Err("CSS_S3_CORS_ORIGINS cannot be empty".to_string());
        }

        Ok(())
    }
}
