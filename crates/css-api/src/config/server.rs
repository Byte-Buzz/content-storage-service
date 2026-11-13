use css_core::infrastructure::config::get_env_or;

/// Http server settings.
///
/// - CSS_SERVER_HOST: host name of the server. Defaults to "0.0.0.0".
/// - CSS_SERVER_PORT: port of the server. Defaults to 8080.
/// - CSS_SERVER_BASE_URL: base url of the server. Defaults to "/".
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub base_url: String,
}

impl ServerConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = Self {
            base_url: get_env_or("SERVER_BASE_URL", "/"),
            host: get_env_or("SERVER_HOST", "0.0.0.0"),
            port: get_env_or("SERVER_PORT", "8080").parse()?,
        };

        if !config.base_url.ends_with("/") {
            config.base_url = format!("{}/", config.base_url);
        }

        config.validate()?;
        Ok(config)
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    fn validate(&self) -> Result<(), String> {
        if self.port == 0 {
            return Err("CSS_SERVER_PORT must be between 1 and 65535".to_string());
        }

        Ok(())
    }
}

/// Grpc server settings.
///
/// - CSS_GRPC_HOST: host name of the server. Defaults to "\[::1\]".
/// - CSS_GRPC_PORT: port of the server. Defaults to 50051.
#[derive(Debug, Clone)]
pub struct GrpcConfig {
    pub host: String,
    pub port: u16,
}

impl GrpcConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Self {
            host: get_env_or("GRPC_HOST", "[::1]"),
            port: get_env_or("GRPC_PORT", "50051").parse()?,
        };

        config.validate()?;
        Ok(config)
    }
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    fn validate(&self) -> Result<(), String> {
        if self.port == 0 {
            return Err("CSS_GRPC_PORT must be between 1 and 65535".to_string());
        }

        Ok(())
    }
}
