use crate::app;
use sqlx::{Pool, Postgres, migrate::Migrator, postgres::PgPoolOptions};

pub static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn create_pool(config: &app::Config) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(64)
        .connect(&config.db.url)
        .await
        .unwrap()
}
