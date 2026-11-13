use axum::{Router, routing::get};

mod check;

pub use check::CheckState;

pub fn create_http_server(services: CheckState) -> Router {
    Router::new()
        .route("/health", get(check::health))
        .route("/ready", get(check::ready))
        .with_state(services)
}
