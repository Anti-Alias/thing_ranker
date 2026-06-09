use thing_ranker::app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    env_logger::init();

    // Gets profile to start application in
    log::info!("Fetching profile");
    let profile = app::AppProfile::from_env();

    // Creates app
    log::info!("Creating application with profile {profile}");
    let app = app::create_app_router(profile).await;

    // Serves app
    log::info!("Serving application");
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
