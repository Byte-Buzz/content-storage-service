use std::fmt::Debug;

use css_core::infrastructure::config::get_env_or;

/// AMQP configuration
///
/// - CSS_AMQP_URL: connection url, required (for example `amqp://css:password@rabbitmq:5672/%2f`).
/// - CSS_AMQP_QUEUE: queue for storage events, declared on startup. Defaults to `css.uploads`.
/// - CSS_AMQP_ROUTING_KEY: routing key binding the queue to `amq.topic`. Defaults to `rustfs.events`.
/// - CSS_AMQP_PREFETCH: maximum number of unacknowledged messages per consumer. Defaults to `16`.
#[derive(Clone)]
pub struct AmqpConfig {
    pub url: String,
    pub queue: String,
    pub routing_key: String,
    pub prefetch: u16,
}

impl Debug for AmqpConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AmqpConfig")
            .field("url", &"...")
            .field("queue", &self.queue)
            .field("routing_key", &self.routing_key)
            .field("prefetch", &self.prefetch)
            .finish()
    }
}

impl AmqpConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Self {
            url: get_env_or("AMQP_URL", ""),
            queue: get_env_or("AMQP_QUEUE", "css.uploads"),
            routing_key: get_env_or("AMQP_ROUTING_KEY", "rustfs.events"),
            prefetch: get_env_or("AMQP_PREFETCH", "16").parse()?,
        };

        config.validate()?;

        Ok(config)
    }

    fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.url.trim().is_empty() {
            return Err("CSS_AMQP_URL cannot be empty".into());
        }

        if self.prefetch == 0 {
            return Err("CSS_AMQP_PREFETCH must be greater than 0".into());
        }

        Ok(())
    }
}
