use std::sync::Arc;

use jwks_client_rs::{JwksClient, source::WebSource};
use sqlx::PgPool;

use crate::{
    app::{AuthConfig, OIDCConfig},
    asset::AssetStore,
};

/// Application state
pub type AppState = Arc<AppStateInner>;

pub struct AppStateInner {
    pub pool: PgPool,
    pub jwks_client: JwksClient<WebSource>,
    pub oidc_config: OIDCConfig,
    pub auth_config: AuthConfig,
    pub asset_store: AssetStore,
}
