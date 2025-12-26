use actix_web::{App, HttpServer, web};
use tokio::{
    signal::unix::{SignalKind, signal},
    sync::oneshot,
};
use tracing_actix_web::TracingLogger;

use crate::infrastructure::{config::Config, logger::init_tracing, storage::S3Client};

mod app;
mod domain;
mod infrastructure;
mod transport;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let config = Config::from_env()?;
    let s3_client = S3Client::new(&config.s3).await?;

    let pg_pool = infrastructure::database::create_pg_pool(&config.database).await?;

    let app = app::App::new(s3_client, pg_pool, config.clone());

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    let grpc_addr = app.config.get_grpc_address().parse()?;
    let grpc_router = transport::grpc::create_grpc_server(&app);

    let grpc = grpc_router.serve_with_shutdown(grpc_addr, async {
        shutdown_rx.await.ok();
    });

    let http_server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(app.services.clone()))
            .configure(transport::http::create_http_server)
    })
    .bind(app.config.get_http_address())?
    .shutdown_timeout(30)
    .run();

    let http_handle = http_server.handle();

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

    let grpc = tokio::spawn(grpc);

    tokio::select! {
        res = grpc => { res??; }
        res = http_server => { res?; }
        _ = shutdown_signal => {
            tracing::info!("shutdown signal received");
        }
    }

    let _ = shutdown_tx.send(());
    http_handle.stop(true).await;

    Ok(())
}
