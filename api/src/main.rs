use thing_ranker::app::{self, Config};
use tokio::net::TcpListener;

const CONFIG_PATH: &str = "config.yml";

#[tokio::main]
async fn main() {
    env_logger::init();
    // Loads application config
    log::info!("Loading config {CONFIG_PATH}");
    let config = Config::load(CONFIG_PATH);
    // Creates app
    let app = app::create_app_router(config).await;
    // Serves app
    log::info!("Serving application on port 8080");
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
