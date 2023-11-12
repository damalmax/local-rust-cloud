use actix_web::dev::Server;
use actix_web::{post, get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use log::LevelFilter;
use serde::Serialize;

use local_rust_cloud_common::request::AwsRequest;
use local_rust_cloud_common::service_handler::ServiceHandler;
use sqlx::migrate::Migrator;

use crate::aws::handlers::action;
use crate::config::AppConfig;

mod aws;
mod config;
mod error;
mod logger;
mod models;
mod repository;
mod secure;

#[cfg(test)]
mod tests;

pub static MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[post("/sts/")]
async fn handle_service_request(body_bytes: web::Bytes, req: HttpRequest) -> impl Responder {
    let aws_request = AwsRequest::from_request(body_bytes, &req);
    return match aws_request {
        Ok(aws_request) => {
            let action_name = aws_request.aws_service_target;
            let action = action::Sts::from_str(&action_name);
            return action.handle(&req, aws_request.query_params);
        }
        Err(e) => {
            let response = Response { message: e.to_string() };
            HttpResponse::BadRequest().json(response)
        }
    };
}

#[derive(Serialize)]
pub struct HealthcheckResponse {
    pub status: String,
}

#[get("/healthcheck")]
async fn handle_healthcheck_request() -> impl Responder {
    let response = HealthcheckResponse {status: "Ok".to_string()};
    HttpResponse::Ok().json(response)
}

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
    log::info!("Starting Local Rust Cloud STS on port {}", app_config.service_port);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(handle_service_request)
            .service(handle_healthcheck_request)
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("0.0.0.0", app_config.service_port))?
    .run();
    return Result::Ok(server);
}
