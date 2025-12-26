use tonic::transport::{Server, server::Router};

use crate::app;

mod uploads;

pub fn create_grpc_server(app: &app::App) -> Router {
    Server::builder().add_service(uploads::create_grpc_service(app))
}
