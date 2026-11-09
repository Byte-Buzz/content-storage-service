use std::time::Duration;

use css_core::infrastructure::config::get_env_or;

/// Upload configuration
///
/// - CSS_UPLOAD_DEFAULT_TTL: default TTL of post url in seconds. Defaults to 900 (15m).
/// - CSS_UPLOAD_MIN_TTL: minimum TTL of post url in seconds. Defaults to 60 (1m).
/// - CSS_UPLOAD_MAX_TTL: maximum TTL of post url in seconds. Defaults to 3600 (1h).
/// - CSS_UPLOAD_MAX_INFO_SIZE: maximum size of info structure. Defaults to 16000 bytes.
#[derive(Debug, Clone)]
pub struct UploadConfig {
    pub default_ttl: Duration,
    pub min_ttl: Duration,
    pub max_ttl: Duration,
    pub max_info_size: usize,
}

impl UploadConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Self {
            default_ttl: Duration::from_secs(get_env_or("UPLOAD_DEFAULT_TTL", "900").parse()?),
            min_ttl: Duration::from_secs(get_env_or("UPLOAD_MIN_TTL", "60").parse()?),
            max_ttl: Duration::from_secs(get_env_or("UPLOAD_MAX_TTL", "3600").parse()?),
            max_info_size: get_env_or("UPLOAD_MAX_INFO_SIZE", "16000").parse()?,
        };

        config.validate()?;

        Ok(config)
    }

    fn validate(&self) -> Result<(), String> {
        if self.min_ttl > self.max_ttl {
            return Err("CSS_UPLOAD_MIN_TTL must be less than CSS_UPLOAD_MAX_TTL".into());
        }

        if self.default_ttl > self.max_ttl {
            return Err("CSS_UPLOAD_DEFAULT_TTL must be less than CSS_UPLOAD_MAX_TTL".into());
        }

        if self.default_ttl < self.min_ttl {
            return Err("default_ttl must be greater than CSS_UPLOAD_MIN_TTL".into());
        }

        if self.max_info_size < 1 {
            return Err("CSS_UPLOAD_MAX_INFO_SIZE must be greater than 0".into());
        }

        Ok(())
    }
}
