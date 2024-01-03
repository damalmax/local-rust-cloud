extern crate core;

use dotenv::dotenv;
use log::LevelFilter;

mod config;
mod http;
mod logger;
#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    logger::init_with_level(LevelFilter::Debug);
    http::server::start(config::EnvironmentAppConfigFactory::new())
        .await
        .expect("Failed to Run HTTP server...")
        .await
}
