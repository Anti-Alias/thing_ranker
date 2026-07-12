use crate::app::{self, DbConfig};
use sqlx::{Pool, Postgres, migrate::Migrator, postgres::PgPoolOptions};

pub static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn create_pool(config: &app::DbConfig) -> Pool<Postgres> {
    let DbConfig {
        name,
        user,
        password,
        host,
        port,
    } = config;
    let url = format!("postgresql://{user}:{password}@{host}:{port}/{name}");
    PgPoolOptions::new()
        .max_connections(64)
        .connect(&url)
        .await
        .unwrap()
}
