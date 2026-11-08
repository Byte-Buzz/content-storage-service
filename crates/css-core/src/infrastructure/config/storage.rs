use std::fmt::Debug;

use super::env::get_env_or;

/// S3 configuration.
///
/// - CSS_S3_PUBLIC_ENDPOINT: public endpoint of the s3 server. Defaults from CSS_S3_ENDPOINT.
/// - CSS_S3_ENDPOINT: endpoint of the s3 server. Defaults to "http://localhost:9000".
/// - CSS_S3_ACCESS_KEY: access key of the s3 server. Required.
/// - CSS_S3_SECRET_KEY: secret key of the s3 server. Required.
/// - CSS_S3_REGION: region of the s3 server. Defaults to "us-east-1".
/// - CSS_S3_TEMP_BUCKET: temporary bucket name of the s3 server. Defaults to "css-temp".
/// - CSS_S3_BUCKET: bucket name of the s3 server. Defaults to "css".
/// - CSS_S3_NOTIFY_ARN: ARN of the notification target for upload events. Subscription is skipped if empty
/// - CSS_S3_TEMP_TTL_DAYS: temporary bucket ttl in days. Defaults to 6.
///   Defaults to 6, long enough to survive a processing outage.
#[derive(Debug, Clone)]
pub struct S3Config {
    pub public_endpoint: String,
    pub endpoint: String,
    pub credentials: SigningCredentials,
    pub temp_bucket: String,
    pub bucket: String,
    pub notify_arn: String,
    pub temp_ttl_days: i32,
}

#[derive(Clone)]
pub struct SigningCredentials {
    pub access_key: String,
    pub secret_key: String,
    pub region: String,
}

impl Debug for SigningCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SigningCredentials { .. }")
    }
}

impl S3Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let endpoint = get_env_or("S3_ENDPOINT", "http://localhost:9000");
        let config = Self {
            public_endpoint: get_env_or("S3_PUBLIC_ENDPOINT", &endpoint),
            endpoint,
            credentials: SigningCredentials {
                access_key: get_env_or("S3_ACCESS_KEY", ""),
                secret_key: get_env_or("S3_SECRET_KEY", ""),
                region: get_env_or("S3_REGION", "us-east-1"),
            },
            temp_bucket: get_env_or("S3_TEMP_BUCKET", "css-temp"),
            bucket: get_env_or("S3_BUCKET", "css"),
            notify_arn: get_env_or("S3_NOTIFY_ARN", ""),
            temp_ttl_days: get_env_or("S3_TEMP_TTL_DAYS", "6").parse()?,
        };

        config.validate()?;

        Ok(config)
    }

    fn validate(&self) -> Result<(), String> {
        if self.public_endpoint.trim().is_empty() {
            return Err("CSS_S3_PUBLIC_ENDPOINT cannot be empty".to_string());
        }

        if self.endpoint.trim().is_empty() {
            return Err("CSS_S3_ENDPOINT cannot be empty".to_string());
        }

        if self.credentials.access_key.trim().is_empty() {
            return Err("CSS_S3_ACCESS_KEY is required".to_string());
        }

        if self.credentials.secret_key.trim().is_empty() {
            return Err("CSS_S3_SECRET_KEY is required".to_string());
        }

        if self.credentials.region.trim().is_empty() {
            return Err("CSS_S3_REGION cannot be empty".to_string());
        }

        if self.temp_bucket.trim().is_empty() {
            return Err("CSS_S3_TEMP_BUCKET cannot be empty".to_string());
        }

        if self.bucket.trim().is_empty() {
            return Err("CSS_S3_BUCKET cannot be empty".to_string());
        }

        if self.temp_ttl_days < 1 {
            return Err("CSS_S3_TEMP_TTL_DAYS must be at least 1".to_string());
        }

        Ok(())
    }
}
