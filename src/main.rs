mod web_server;
mod infra;
mod dao;
mod api;
mod logic;
mod repository;

#[tokio::main]
async fn main() {
    let db_url: &str = "mysql://root:admin123@localhost:3306/short_url_srv";
    infra::db::init(db_url,10).await;

    infra::logger::init();
    
    web_server::service::build_and_run().await;
}