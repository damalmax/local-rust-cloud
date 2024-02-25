use crate::config::AppConfig;
use axum::routing::post;
use axum::Router;

use crate::http::aws;

pub(crate) async fn router(app_config: &AppConfig) -> std::io::Result<Router> {
    let iam_db = local_cloud_db::LocalDb::new(&app_config.database_url, &sqlx::migrate!())
        .await
        .map_err(|err| {
            log::error!("Failed to setup DB: {}", err);
            err
        })
        .unwrap();

    // setting up HTTP Router
    let app = Router::new().route("/sts/", post(aws::sts::handle)).with_state(iam_db);

    Ok(app)
}
