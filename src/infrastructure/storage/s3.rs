use s3::creds::Credentials;

use crate::infrastructure::config::S3Config;

#[derive(Debug, Clone)]
pub struct S3Client {
    pub temp_bucket: Box<s3::Bucket>,
    pub bucket: Box<s3::Bucket>,
}

impl S3Client {
    pub async fn new(config: &S3Config) -> Result<Self, Box<dyn std::error::Error>> {
        let region_provider = s3::Region::Custom {
            region: "custom".to_string(),
            endpoint: config.endpoint.clone(),
        };

        let credentials_provider = Credentials::new(
            Some(&config.access_key),
            Some(&config.secret_key),
            None,
            None,
            None,
        )?;

        let temp_bucket = s3::Bucket::new(
            &config.temp_bucket,
            region_provider.clone(),
            credentials_provider.clone(),
        )?
        .with_path_style();

        let bucket = s3::Bucket::new(&config.bucket, region_provider, credentials_provider)?
            .with_path_style();

        Ok(S3Client {
            temp_bucket: temp_bucket,
            bucket: bucket,
        })
    }
}
