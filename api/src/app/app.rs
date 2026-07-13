//! Contains logic that creates the application router, which is the heart of the REST api.

use std::sync::Arc;

use crate::app::{AppStateInner, AssetStoreType, Config};
use crate::asset::AssetStore;
use crate::layer::auth::authenticate;
use crate::{account, category, db, thing};
use axum::http::{HeaderValue, header};
use axum::routing::{get, post};
use axum::{Router, middleware};
use jwks_client_rs::JwksClient;
use jwks_client_rs::source::WebSource;
use reqwest::Url;
use tower_http::cors::{AllowHeaders, AllowMethods, CorsLayer};
use tower_http::services::ServeDir;

const GOOGLE_JWKS_URL: &str = "https://www.googleapis.com/oauth2/v3/certs";

/// Creates the application router with a given configuration
pub async fn create_app_router(config: Config) -> Router {
    // Connects to database and runs migrations
    log::info!("Connecting to DB");
    let pool = db::create_pool(&config.db).await;
    log::info!("Running DB migrations");
    db::MIGRATOR.run(&pool).await.unwrap();
    // Sets up JWKS client for token validation
    let jwks_client = create_jwks_client();
    let asset_store = match config.asset_store_type {
        AssetStoreType::Local => AssetStore::local(),
        AssetStoreType::S3 => AssetStore::s3(),
    };
    // Sets up app state
    let state = Arc::new(AppStateInner {
        pool,
        jwks_client,
        auth_config: config.auth,
        oidc_config: config.oidc,
        asset_store,
    });
    // Sets up auth layer
    let auth_layer = middleware::from_fn_with_state(state.clone(), authenticate);
    // Sets up cors layer
    let allow_headers = AllowHeaders::list([header::AUTHORIZATION]);
    let allow_origin: HeaderValue = config.cors.allowed_origin.parse().unwrap();
    let cors_layer = CorsLayer::new()
        .allow_methods(AllowMethods::any())
        .allow_headers(allow_headers)
        .allow_origin(allow_origin);
    // Constructs app router
    Router::new()
        .route("/api/things", post(thing::create_thing))
        .route("/api/categories", post(category::create_category))
        .route_layer(auth_layer)
        .route("/api/categories/{category_id}", get(category::get_category))
        .route("/api/things", get(thing::get_thing_page))
        .route("/api/things/{thing_id}", get(thing::get_thing))
        .route("/api/account/token", post(account::create_token))
        .route("/api/health", get(health))
        .layer(cors_layer)
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(state)
}

fn create_jwks_client() -> JwksClient<WebSource> {
    let jwks_url = Url::parse(GOOGLE_JWKS_URL).unwrap();
    let source = WebSource::builder().build(jwks_url).unwrap();
    JwksClient::builder().build(source)
}

async fn health() -> &'static str {
    "Application is running"
}
