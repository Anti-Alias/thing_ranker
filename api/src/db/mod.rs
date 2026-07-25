use crate::app::{self, DbConfig};
use sqlx::{Pool, Postgres, migrate::Migrator, postgres::PgPoolOptions};

pub static MIGRATOR: Migrator = sqlx::migrate!();

/// Creates a database connection pool.
/// Runs migrations if configured.
pub async fn create_pool(config: &app::DbConfig, migrate: bool) -> Pool<Postgres> {
    let DbConfig {
        name,
        user,
        password,
        host,
        port,
    } = config;
    let url = format!("postgresql://{user}:{password}@{host}:{port}/{name}");
    let pool = PgPoolOptions::new()
        .max_connections(64)
        .connect(&url)
        .await
        .unwrap();
    if migrate {
        log::info!("Running DB migrations");
        MIGRATOR.run(&pool).await.unwrap();
    }
    pool
}
