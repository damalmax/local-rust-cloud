use dotenv::dotenv;
use log::LevelFilter;
use tokio::net::TcpListener;

mod config;
mod error;
mod logger;
mod secure;

mod http;
#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    logger::init_with_level(LevelFilter::Debug);

    let app_config = config::AppConfig::parse_env();
    let app = http::server::router(&app_config)
        .await
        .expect("Failed to setup HTTP server...");

    // start HTTP server
    let listener = TcpListener::bind(("0.0.0.0", app_config.service_port)).await.unwrap();

    tracing::info!("Starting Local Rust Cloud STS on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
