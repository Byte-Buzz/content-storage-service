use aws_config::{BehaviorVersion, meta::region::RegionProviderChain};
use aws_sdk_s3::{Client, config::Credentials};

use crate::infrastructure::config::S3Config;

/// Creates an S3 client from a given S3 configuration.
///
/// This function takes an S3 configuration and returns an S3 client.
/// The client is created using the AWS SDK defaults, with the given
/// configuration overriding the default endpoint URL and credentials.
pub async fn create_s3_client(config: &S3Config) -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .credentials_provider(Credentials::new(
            config.access_key.clone(),
            config.secret_key.clone(),
            None,
            None,
            "rustfs",
        ))
        .endpoint_url(config.endpoint.clone())
        .load()
        .await;

    Client::new(&config)
}
