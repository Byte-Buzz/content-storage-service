use aws_sdk_s3::types::{
    Event, ExpirationStatus,
    builders::{
        BucketLifecycleConfigurationBuilder, CorsConfigurationBuilder, CorsRuleBuilder,
        LifecycleExpirationBuilder, LifecycleRuleBuilder, LifecycleRuleFilterBuilder,
        NotificationConfigurationBuilder, QueueConfigurationBuilder,
    },
};
use css_core::infrastructure::{
    config::S3Config,
    storage::{S3Client, create_s3_client},
};
use serde_json::json;

pub async fn run_storage(config: S3Config) -> Result<(), Box<dyn std::error::Error>> {
    let client = create_s3_client(&config, config.endpoint.clone()).await;

    tracing::info!(bucket = %config.bucket, "creating bucket");

    ensure_bucket(&client, &config.bucket).await?;

    tracing::info!(bucket = %config.bucket, "creating bucket policy");

    client
        .put_bucket_policy()
        .bucket(&config.bucket)
        .policy(
            json!({
              "Version": "2012-10-17",
              "Statement": [{
                "Effect": "Allow",
                "Principal": "*",
                "Action": "s3:GetObject",
                "Resource": format!("arn:aws:s3:::{}/*", config.bucket),
                "Condition": {"StringEquals": {"s3:ExistingObjectTag/visibility": "public"}}
              }]
            })
            .to_string(),
        )
        .send()
        .await?;

    tracing::info!(bucket = %config.temp_bucket, "creating bucket");

    ensure_bucket(&client, &config.temp_bucket).await?;

    tracing::info!(bucket = %config.temp_bucket, "creating lifecycle configuration");

    client
        .put_bucket_lifecycle_configuration()
        .bucket(&config.temp_bucket)
        .lifecycle_configuration(
            BucketLifecycleConfigurationBuilder::default()
                .rules(
                    LifecycleRuleBuilder::default()
                        .id("expire-temp")
                        .filter(LifecycleRuleFilterBuilder::default().build())
                        .expiration(
                            LifecycleExpirationBuilder::default()
                                .days(config.temp_ttl_days)
                                .build(),
                        )
                        .status(ExpirationStatus::Enabled)
                        .build()?,
                )
                .build()?,
        )
        .send()
        .await?;

    tracing::info!(bucket = %config.temp_bucket, "creating upload notification settings");

    if config.notify_arn.trim().is_empty() {
        tracing::warn!("no notify arn configured, skipping notification setup");
    } else {
        client
            .put_bucket_notification_configuration()
            .bucket(&config.temp_bucket)
            .notification_configuration(
                NotificationConfigurationBuilder::default()
                    .queue_configurations(
                        QueueConfigurationBuilder::default()
                            .queue_arn(config.notify_arn)
                            .events(Event::S3ObjectCreated)
                            .build()?,
                    )
                    .build(),
            )
            .send()
            .await?;
    }

    tracing::info!(bucket = %config.temp_bucket, "creating cors settings");

    if config.cors_origins.iter().any(|o| o == "*") {
        tracing::warn!("cors allows any origin, set CSS_S3_CORS_ORIGINS in production");
    }
    client
        .put_bucket_cors()
        .bucket(&config.temp_bucket)
        .cors_configuration(
            CorsConfigurationBuilder::default()
                .cors_rules(
                    CorsRuleBuilder::default()
                        .allowed_methods("POST")
                        .allowed_headers("*")
                        .set_allowed_origins(Some(config.cors_origins))
                        .max_age_seconds(3600)
                        .build()?,
                )
                .build()?,
        )
        .send()
        .await?;

    Ok(())
}

async fn ensure_bucket(client: &S3Client, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    match client.create_bucket().bucket(name).send().await {
        Ok(_) => Ok(()),
        Err(e)
            if e.as_service_error()
                .is_some_and(|s| s.is_bucket_already_owned_by_you()) =>
        {
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}
