use axum;
use tokio::net::TcpListener;

pub async fn build_and_run() {
    let app = super::router::new();

    let listener = TcpListener::bind("0.0.0.0:10240").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}