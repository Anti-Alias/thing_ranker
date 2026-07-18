use config::File;
use serde::Deserialize;

/// Application configuration
#[derive(Deserialize, Debug)]
pub struct Config {
    pub port: u32,
    pub asset_store_type: AssetStoreType,
    pub db: DbConfig,
    pub cors: CorsConfig,
    pub auth: AuthConfig,
    pub oidc: OIDCConfig,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AssetStoreType {
    Local,
    S3,
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
