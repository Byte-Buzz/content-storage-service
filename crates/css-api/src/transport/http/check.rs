use std::time::Duration;

use aws_sdk_s3::Client;
use axum::{extract::State, http::StatusCode};
use sqlx::PgPool;

#[derive(Clone)]
pub struct CheckState {
    pub pg_pool: PgPool,
    pub s3_client: Client,
    pub buckets: Vec<String>,
}

pub async fn health() -> StatusCode {
    StatusCode::OK
}

pub async fn ready(State(state): State<CheckState>) -> Result<StatusCode, StatusCode> {
    let storage = async {
        for bucket in &state.buckets {
            check_bucket(&state.s3_client, bucket).await?;
        }
        Ok::<(), StatusCode>(())
    };

    timeout(async { tokio::try_join!(storage, check_db(&state.pg_pool)).map(|_| ()) }).await?;

    Ok(StatusCode::OK)
}

async fn timeout<F>(f: F) -> Result<(), StatusCode>
where
    F: Future<Output = Result<(), StatusCode>>,
{
    match tokio::time::timeout(Duration::from_secs(2), f).await {
        Ok(res) => res,
        Err(e) => {
            tracing::warn!(error = %e, "reached checking timeout");
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

async fn check_bucket(s3: &Client, bucket: &str) -> Result<(), StatusCode> {
    match s3.head_bucket().bucket(bucket).send().await {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::warn!(bucket, error = %e, "storage check failed");
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

async fn check_db(pg_pool: &PgPool) -> Result<(), StatusCode> {
    match sqlx::query("SELECT 1").fetch_one(pg_pool).await {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::warn!(error = %e, "database check failed");
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}
