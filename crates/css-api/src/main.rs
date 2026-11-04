use css_core::infrastructure::{
    database::create_pg_pool, logger::init_tracing, storage::create_s3_client,
};
use tokio::{
    signal::unix::{SignalKind, signal},
    sync::watch,
};

use crate::config::Config;

mod app;
mod config;
mod transport;

type BoxError = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    init_tracing();

    let config = Config::from_env()?;
    let s3_client = create_s3_client(&config.s3).await;

    let pg_pool = create_pg_pool(&config.database).await?;

    let app = app::App::new(s3_client, pg_pool, config.clone());

    let grpc_addr = app.config.grpc.address().parse()?;
    let grpc_router = transport::grpc::create_grpc_server(&app);

    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let mut grpc_rx = shutdown_rx.clone();
    let grpc = grpc_router.serve_with_shutdown(grpc_addr, async move {
        let _ = grpc_rx.changed().await;
    });

    let state = transport::http::CheckState {
        pg_pool: app.pg_pool.clone(),
        s3_client: app.s3_client.clone(),
        buckets: vec![
            app.config.s3.bucket.clone(),
            app.config.s3.temp_bucket.clone(),
        ],
    };

    let router = transport::http::create_http_server(state);
    let listener = tokio::net::TcpListener::bind(app.config.server.address()).await?;

    let mut http_rx = shutdown_rx.clone();
    let http = axum::serve(listener, router).with_graceful_shutdown(async move {
        let _ = http_rx.changed().await;
    });

    let shutdown_signal = async {
        let mut sigterm =
            signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
        let mut sigint = signal(SignalKind::interrupt()).expect("failed to install SIGINT handler");

        tokio::select! {
            _ = sigterm.recv() => {
                tracing::info!("SIGTERM received");
            }
            _ = sigint.recv() => {
                tracing::info!("SIGINT received");
            }
        }
    };

    let signal = async move {
        shutdown_signal.await;
        let _ = shutdown_tx.send(true);
        Ok::<(), BoxError>(())
    };

    tokio::try_join!(
        async { grpc.await.map_err(BoxError::from) },
        async { http.await.map_err(BoxError::from) },
        signal,
    )?;

    Ok(())
}
