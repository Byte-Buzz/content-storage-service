use std::time::Duration;

use futures_lite::StreamExt;
use lapin::{
    Channel,
    options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions},
    types::FieldTable,
};
use tokio::sync::watch;

use super::event::Event;
use crate::app::service::{StorageEventService, error::ServiceError};

pub async fn run(
    channel: Channel,
    queue: &str,
    service: StorageEventService,
    mut shutdown: watch::Receiver<bool>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut consumer = channel
        .basic_consume(
            queue.into(),
            "css-api".into(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    loop {
        tokio::select! {
            _ = shutdown.changed() => {
                return Ok(());
            }
            event = consumer.next() => match event {
                None => return Err("amqp consumer stream closed".into()),
                Some(Err(e)) => { tracing::warn!(error = %e, "amqp delivery error"); }
                Some(Ok(delivery)) => match handle(&service, &delivery.data).await {
                    Outcome::Ack => { delivery.ack(BasicAckOptions::default()).await?; }
                    Outcome::Requeue => {
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        delivery.nack(BasicNackOptions { requeue: true, ..Default::default() }).await?;
                    }
                },
            }
        }
    }
}

enum Outcome {
    Ack,
    Requeue,
}

async fn handle(service: &StorageEventService, body: &[u8]) -> Outcome {
    let event: Event = match serde_json::from_slice(body) {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!(error = %e, "malformed storage event");
            return Outcome::Ack;
        }
    };

    let objects = match event.into_objects() {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!(error = %e, "invalid storage event");
            return Outcome::Ack;
        }
    };

    for object in objects {
        if !object.event_name.starts_with("s3:ObjectCreated:") {
            tracing::debug!(event = %object.event_name, "skipping storage event");
            continue;
        }

        match service
            .handle_object_created(&object.bucket, &object.key)
            .await
        {
            Ok(()) => {}
            Err(e @ ServiceError::InvalidArgument(_)) => {
                tracing::warn!(error = %e, key = %object.key, "rejected storage event");
            }
            Err(e) => {
                tracing::error!(error = ?e, "failed to handle storage event");
                return Outcome::Requeue;
            }
        }
    }

    Outcome::Ack
}
