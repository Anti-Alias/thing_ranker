use config::File;
use serde::Deserialize;

use crate::account::AccountSettings;

const CONFIG_PATH: &str = "config.yml";

/// Application configuration.
/// Read from config.yml file(s).
#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub port: u32,
    pub asset_store_type: AssetStoreType,
    pub db: DbConfig,
    pub cors: CorsConfig,
    pub auth: AuthConfig,
    pub oidc: OIDCConfig,
    pub account_settings: Vec<AccountSettings>,
}

#[derive(Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AssetStoreType {
    Local,
    S3,
}

/// Config values for database connectivity
#[derive(Deserialize, Clone, Debug)]
pub struct DbConfig {
    pub name: String,
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u32,
}

/// Config values for CORS
#[derive(Deserialize, Clone, Debug)]
pub struct CorsConfig {
    pub allowed_origin: String,
}

/// Config values for authenticating accounts
#[derive(Deserialize, Clone, Debug)]
pub struct AuthConfig {
    /// Secret used to sign and validate JWTs
    pub jwt_secret: String,
    /// Long long before JWTs expire
    pub jwt_exp_secs: i64,
}

/// Config values for dealing with identity providers through the OIDC protocol
#[derive(Deserialize, Clone, Debug)]
pub struct OIDCConfig {
    pub google: GoogleConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GoogleConfig {
    pub client_id: String,
}

impl Config {
    /// Loads config from a file
    pub fn load() -> Self {
        let builder = config::Config::builder().add_source(File::with_name(CONFIG_PATH));
        builder.build().unwrap().try_deserialize::<Self>().unwrap()
    }
}
