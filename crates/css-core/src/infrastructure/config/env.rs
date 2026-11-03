/// Get an environment variable with the "CSS_" prefix.
pub fn get_env(key: &str) -> Result<String, std::env::VarError> {
    std::env::var(format!("CSS_{key}"))
}

/// Get an environment variable with the "CSS_" prefix or a default value.
pub fn get_env_or(key: &str, default: &str) -> String {
    get_env(key).unwrap_or_else(|_| default.to_string())
}
