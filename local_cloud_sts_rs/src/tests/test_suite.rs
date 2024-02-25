use uuid::Uuid;

use crate::config::AppConfig;

pub(crate) async fn start_server(port: u16) -> std::io::Result<axum::Router> {
    let db_file_name = Uuid::new_v4();
    let app_config = AppConfig {
        database_url: format!("file:{}?mode=memory&cache=shared", db_file_name),
        service_port: port,
    };
    crate::http::server::router(&app_config).await
}
