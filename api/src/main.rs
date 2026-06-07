use thing_ranker::create_app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let router = create_app();
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
