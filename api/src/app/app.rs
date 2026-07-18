//! Contains logic that creates the application router, which is the heart of the REST api.

use std::sync::Arc;

use crate::account::{AccountRole, upsert_account};
use crate::app::{AppStateInner, AssetStoreType, Config};
use crate::asset::AssetStore;
use crate::layer::auth::{authenticate, authorize_admin};
use crate::{account, category, db, thing};
use axum::http::{HeaderValue, header};
use axum::routing::{get, post};
use axum::{Router, middleware};
use jwks_client_rs::JwksClient;
use jwks_client_rs::source::WebSource;
use reqwest::Url;
use sqlx::PgPool;
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
    // Upserts accounts from config
    log::info!("Upserting accounts");
    apply_account_roles(&config.roles, &pool).await;
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
    // Sets up auth layers
    let authenticate_layer = middleware::from_fn_with_state(state.clone(), authenticate);
    let admin_layer = middleware::from_fn(authorize_admin);
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
        .route_layer(admin_layer)
        .route_layer(authenticate_layer)
        .route("/api/categories/{category_id}", get(category::get_category))
        .route("/api/things", get(thing::get_thing_page))
        .route("/api/things/{thing_id}", get(thing::get_thing))
        .route("/api/account/token", post(account::exchange_idp_token))
        .route("/api/health", get(health))
        .layer(cors_layer)
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(state)
}

pub async fn apply_account_roles(account_roles: &[AccountRole], pool: &PgPool) {
    for account_role in account_roles {
        upsert_account(&account_role.email, account_role.role, pool)
            .await
            .unwrap();
    }
}

fn create_jwks_client() -> JwksClient<WebSource> {
    let jwks_url = Url::parse(GOOGLE_JWKS_URL).unwrap();
    let source = WebSource::builder().build(jwks_url).unwrap();
    JwksClient::builder().build(source)
}

async fn health() -> &'static str {
    "Application is running"
}
