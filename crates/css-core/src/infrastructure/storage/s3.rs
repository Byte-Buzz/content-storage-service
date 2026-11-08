use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{Client, config::Credentials};

use crate::infrastructure::{config::S3Config, storage::S3Client};

/// Creates an S3 client from a given S3 configuration.
///
/// This function takes an S3 configuration and returns an S3 client.
/// The client is created using the AWS SDK defaults, with the given
/// configuration overriding the default endpoint URL and credentials.
pub async fn create_s3_client(config: &S3Config) -> S3Client {
    let shared = aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(Credentials::new(
            config.credentials.access_key.clone(),
            config.credentials.secret_key.clone(),
            None,
            None,
            "rustfs",
        ))
        .endpoint_url(config.endpoint.clone())
        .region(Region::new(config.credentials.region.clone()))
        .load()
        .await;

    let s3_config = aws_sdk_s3::config::Builder::from(&shared)
        .force_path_style(true)
        .build();

    Client::from_conf(s3_config)
}
