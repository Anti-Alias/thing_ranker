//! Contains logic that creates the application router, which is the heart of the REST api.

use axum::Router;
use axum::routing::get;

pub fn create_app() -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> &'static str {
    "Hello, world"
}
