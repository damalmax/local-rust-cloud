use dotenv::dotenv;
use log::LevelFilter;

use crate::config::EnvironmentAppConfigFactory;

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
    http::server::start(EnvironmentAppConfigFactory::new())
        .await
        .expect("Failed to Run HTTP server...")
        .await
}
