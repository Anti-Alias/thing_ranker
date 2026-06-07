use thing_ranker::{app, db};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    env_logger::init();

    let env = app::Env::from_env();
    log::info!("Loading application config for {env} environment");
    let config = app::Config::load(env);

    log::info!("Connecting to DB");
    let pool = db::create_pool(&config).await;
    log::info!("Running DB migrations");
    db::MIGRATOR.run(&pool).await.unwrap();

    // Creates APP and serves it
    log::info!("Starting application");
    let app = app::create_app();
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
