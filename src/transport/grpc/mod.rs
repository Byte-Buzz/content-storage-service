use tonic::transport::{Server, server::Router};

use crate::app;

mod content;

pub fn create_grpc_server(app: &app::App) -> Router {
    Server::builder().add_service(content::create_grpc_service(app))
}
