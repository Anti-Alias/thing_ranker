//! Contains logic that creates the application router, which is the heart of the REST api.

use crate::app::{AppProfile, AppState, Config, validate_claims};
use crate::{account, category, db, thing};
use axum::Router;
use axum::routing::{get, post};
use tower_http::cors::{self, AllowHeaders, AllowMethods, CorsLayer};

/// Creates the application router for a given profile.
/// Configuration is loaded from a base config file, and an environment-specific config file.
pub async fn create_app_router(profile: AppProfile) -> Router {
    // Loads application config
    log::info!("Loading application config with {profile} profile");
    let config = Config::load(profile);
    // Connects to database and runs migrations
    log::info!("Connecting to DB");
    let pool = db::create_pool(&config).await;
    let state = AppState { pool };
    log::info!("Running DB migrations");
    db::MIGRATOR.run(&state.pool).await.unwrap();
    // Sets up cors layer
    let cors_layer = CorsLayer::new()
        .allow_methods(AllowMethods::any())
        .allow_origin(cors::Any)
        .allow_headers(AllowHeaders::any());
    // Constructs app router
    Router::new()
        .route("/account/token", post(account::create_token))
        .layer(axum::middleware::from_fn(validate_claims))
        .route("/health", get(health))
        .route("/categories/{category_id}", get(category::get_category))
        .route("/categories", post(category::create_category))
        .route("/things/{thing_id}", get(thing::get_thing))
        .route("/things", post(thing::create_thing))
        .layer(cors_layer)
        .with_state(state)
}

async fn health() -> &'static str {
    "Application is running"
}
