use crate::infrastructure::config::Config;

/// Validates a Config instance.
///
/// This function checks that all required fields in the config are present and valid.
/// If any field is invalid, it returns an error string explaining what is wrong.
/// If all fields are valid, it returns Ok(()).
pub fn validate_config(config: &Config) -> Result<(), String> {
    // Validate DatabaseConfig
    if config.database.url.trim().is_empty() {
        return Err("Database URL cannot be empty".to_string());
    }

    // Validate ServerConfig
    if config.server.port == 0 {
        return Err("Server port must be between 1 and 65535".to_string());
    }

    // Validate S3Config
    if config.s3.endpoint.trim().is_empty() {
        return Err("S3 endpoint cannot be empty".to_string());
    }
    if config.s3.access_key.trim().is_empty() {
        return Err("S3 access key cannot be empty".to_string());
    }
    if config.s3.secret_key.trim().is_empty() {
        return Err("S3 secret key cannot be empty".to_string());
    }

    Ok(())
}
