use config::File;
use serde::Deserialize;
use sqlx::PgPool;
use std::{env, fmt};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub db: DbConfig,
}

#[derive(Deserialize, Debug)]
pub struct DbConfig {
    pub url: String,
}

impl Config {
    /// Loads app configuration from a base file, and an environment-specific file.
    pub fn load(profile: AppProfile) -> Self {
        const BASE_FILE_SOURCE: &str = "config/base.yml";
        let env_file_source = match profile {
            AppProfile::Local => "config/local.yml",
            AppProfile::Dev => "config/dev.yml",
            AppProfile::Prod => "config/prod.yml",
        };
        log::info!("Loading config files {BASE_FILE_SOURCE} and {env_file_source}");
        let builder = config::Config::builder()
            .add_source(File::with_name(BASE_FILE_SOURCE))
            .add_source(File::with_name(env_file_source));
        builder.build().unwrap().try_deserialize::<Self>().unwrap()
    }
}

/// Application state
#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: PgPool,
}

/// Environment of the application
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum AppProfile {
    #[default]
    Local,
    Dev,
    Prod,
}

impl fmt::Display for AppProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppProfile::Local => write!(f, "Local"),
            AppProfile::Dev => write!(f, "Dev"),
            AppProfile::Prod => write!(f, "Prod"),
        }
    }
}

impl AppProfile {
    pub fn from_env() -> Self {
        let profile = env::var("APP_PROFILE").unwrap_or_else(|_| String::from("Local"));
        match profile.as_str() {
            "Local" => Self::Local,
            "Dev" => Self::Dev,
            "Prod" => Self::Prod,
            _ => panic!("Unexpected profile. Valid values are Local, Dev, Prod"),
        }
    }
}
