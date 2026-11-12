use std::time::Duration;

use lapin::{
    Channel, Connection, ConnectionProperties,
    options::{BasicQosOptions, QueueBindOptions, QueueDeclareOptions},
    types::{AMQPValue, FieldTable, ShortString},
};

use crate::config::AmqpConfig;

mod consumer;
mod event;

pub use consumer::run;

pub async fn connect(
    config: &AmqpConfig,
) -> Result<(Connection, Channel), Box<dyn std::error::Error>> {
    let conn = tokio::time::timeout(
        Duration::from_secs(10),
        Connection::connect(
            &config.url,
            ConnectionProperties::default().enable_auto_recover(),
        ),
    )
    .await??;

    let channel = conn.create_channel().await?;
    let queue_name: ShortString = config.queue.clone().into();

    let mut args = FieldTable::default();
    args.insert(
        "x-queue-type".into(),
        AMQPValue::LongString("quorum".into()),
    );

    channel
        .queue_declare(queue_name.clone(), QueueDeclareOptions::durable(), args)
        .await?;

    channel
        .queue_bind(
            queue_name,
            "amq.topic".into(),
            config.routing_key.clone().into(),
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    channel
        .basic_qos(config.prefetch, BasicQosOptions::default())
        .await?;

    Ok((conn, channel))
}
