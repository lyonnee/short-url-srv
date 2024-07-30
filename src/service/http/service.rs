use axum;
use tokio::net::TcpListener;

use crate::{api::http::router, infra::config};

pub async fn build_and_run() {
    let app = router::new();

    let lock_config = config::get_configs();
    let config = lock_config.as_ref().unwrap();

    let listener = TcpListener::bind(&config.http.addr).await.unwrap();

    tracing::info!(
        "The web server has already running, addr: {}",
        &config.http.addr
    );

    axum::serve(listener, app).await.unwrap();
}
