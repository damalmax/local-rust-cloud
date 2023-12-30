use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

use crate::config::AppConfigFactory;
use crate::http::aws;

pub(crate) async fn start(app_config_factory: impl AppConfigFactory) -> std::io::Result<Server> {
    let app_config = app_config_factory.get_config();
    // connect to DB
    let sts_db = local_cloud_db::LocalDb::new(&app_config.database_url, &sqlx::migrate!())
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
            .route("/iam/", web::post().to(aws::iam::handle))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("0.0.0.0", app_config.service_port))?
    .run();
    return Ok(server);
}
