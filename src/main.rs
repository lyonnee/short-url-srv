use std::env;

mod api;
mod dao;
mod infra;
mod logic;
mod repository;
mod service;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let env: String = get_env(&args);
    let _ = infra::config::init(env);

    let _guard = infra::logger::init();

    infra::db::init().await;

    service::http::service::build_and_run().await;
}

fn get_env(args: &[String]) -> String {
    args[1].clone()
}
