use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::LevelFilter;

use sqlx::migrate::Migrator;

use crate::config::AppConfig;

mod aws;
mod config;
mod error;
mod handlers;
mod logger;
mod models;
mod repository;

#[cfg(test)]
mod tests;

pub static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    logger::init_with_level(LevelFilter::Debug);
    create_http_server(|| AppConfig::init())
        .await
        .expect("Failed to Run HTTP server...")
        .await
}

async fn create_http_server(app_config_factory: impl Fn() -> AppConfig) -> std::io::Result<Server> {
    let app_config = app_config_factory();
    // connect to DB
    let sts_db = local_rust_cloud_sqlite::Database::new(&app_config.database_url, &MIGRATOR)
        .await
        .map_err(|err| {
            log::error!("Failed to setup DB: {}", err);
            err
        })
        .unwrap();

    let app_data = web::Data::new(sts_db);

    // start HTTP server
    log::info!("Starting Local Rust Cloud IAM on port {}", app_config.service_port);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(handlers::iam::handle)
            .service(handlers::healthcheck::handle)
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("0.0.0.0", app_config.service_port))?
    .run();
    return Result::Ok(server);
}
