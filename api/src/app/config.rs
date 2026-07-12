use std::sync::Arc;

use config::File;
use jwks_client_rs::{JwksClient, source::WebSource};
use serde::Deserialize;
use sqlx::PgPool;

/// Application configuration
#[derive(Deserialize, Debug)]
pub struct Config {
    pub port: u32,
    pub db: DbConfig,
    pub cors: CorsConfig,
    pub auth: AuthConfig,
    pub oidc: OIDCConfig,
}

/// Config values for database connectivity
#[derive(Deserialize, Debug)]
pub struct DbConfig {
    pub name: String,
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u32,
}

/// Config values for CORS
#[derive(Deserialize, Debug)]
pub struct CorsConfig {
    pub allowed_origin: String,
}

/// Config values for authenticating accounts
#[derive(Deserialize, Debug)]
pub struct AuthConfig {
    /// Secret used to sign and validate JWTs
    pub jwt_secret: String,
    /// Long long before JWTs expire
    pub jwt_exp_secs: i64,
}

/// Config values for dealing with identity providers through the OIDC protocol
#[derive(Deserialize, Debug)]
pub struct OIDCConfig {
    pub google: GoogleConfig,
}

#[derive(Deserialize, Debug)]
pub struct GoogleConfig {
    pub client_id: String,
}

impl Config {
    /// Loads app configuration from a base file, and an environment-specific file.
    pub fn load(config_path: &str) -> Self {
        let builder = config::Config::builder().add_source(File::with_name(config_path));
        builder.build().unwrap().try_deserialize::<Self>().unwrap()
    }
}

/// Application state
pub type AppState = Arc<AppStateInner>;

pub struct AppStateInner {
    pub pool: PgPool,
    pub jwks_client: JwksClient<WebSource>,
    pub oidc_config: OIDCConfig,
    pub auth_config: AuthConfig,
}
