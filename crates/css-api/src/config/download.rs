use std::time::Duration;

use css_core::infrastructure::config::get_env_or;

/// Download configuration.
///
/// - CSS_DOWNLOAD_URL_WINDOW: signing window in seconds; URLs stay valid for two windows.
///   Defaults to 900. Must be between 31 and 302400 (3.5 days).
/// - CSS_DOWNLOAD_MAX_NAME_LENGTH: maximum download name length in characters.
///   Longer names are truncated, keeping the extension. Defaults to 200.
/// - CSS_DOWNLOAD_MAX_ITEMS: maximum number of items per request. Defaults to 1000
///   (for example, 250 files with 4 variants each).
#[derive(Debug, Clone)]
pub struct DownloadConfig {
    pub url_window: Duration,
    pub max_name_length: u32,
    pub max_items: u32,
}

impl DownloadConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Self {
            url_window: Duration::from_secs(get_env_or("DOWNLOAD_URL_WINDOW", "900").parse()?),
            max_name_length: get_env_or("DOWNLOAD_MAX_NAME_LENGTH", "200").parse()?,
            max_items: get_env_or("DOWNLOAD_MAX_ITEMS", "1000").parse()?,
        };

        config.validate()?;

        Ok(config)
    }

    fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.url_window.as_secs() < 31 {
            return Err("CSS_DOWNLOAD_URL_WINDOW must be greater than 30".into());
        }

        if self.max_name_length < 1 {
            return Err("CSS_DOWNLOAD_MAX_NAME_LENGTH must be greater than 0".into());
        }

        if self.max_items < 1 {
            return Err("CSS_DOWNLOAD_MAX_ITEMS must be greater than 0".into());
        }

        Ok(())
    }
}
