//! Contains logic that creates the application router, which is the heart of the REST api.

use axum::Router;
use axum::routing::get;
use config::File;
use serde::Deserialize;
use std::{env, fmt};

pub fn create_app() -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> &'static str {
    "Hello, world"
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub db_url: String,
}

impl Config {
    pub fn load(environment: Env) -> Self {
        const BASE_FILE_SOURCE: &str = "config/base.yml";
        let env_file_source = match environment {
            Env::Local => "config/local.yml",
            Env::Dev => "config/dev.yml",
            Env::Prod => "config/prod.yml",
        };
        log::info!("Loading config files {BASE_FILE_SOURCE} and {env_file_source}");
        let builder = config::Config::builder()
            .add_source(File::with_name(BASE_FILE_SOURCE))
            .add_source(File::with_name(env_file_source));
        builder.build().unwrap().try_deserialize::<Self>().unwrap()
    }
}

/// Environment of the application
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum Env {
    #[default]
    Local,
    Dev,
    Prod,
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Env::Local => write!(f, "Local"),
            Env::Dev => write!(f, "Dev"),
            Env::Prod => write!(f, "Prod"),
        }
    }
}

impl Env {
    pub fn from_env() -> Self {
        let env = env::var("APP_ENV").unwrap_or_else(|_| String::from("Local"));
        match env.as_str() {
            "Local" => Self::Local,
            "Dev" => Self::Dev,
            "Prod" => Self::Prod,
            _ => panic!("Unexpected environment. Valid values are Local, Dev, Prod"),
        }
    }
}
