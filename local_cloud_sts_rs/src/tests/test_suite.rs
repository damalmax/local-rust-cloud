use crate::config::{AppConfig, AppConfigFactory};
use actix_server::Server;
use local_cloud_testing::suite::TestAppConfig;

pub(crate) struct TestAppConfigFactory {
    config: TestAppConfig,
}

impl AppConfigFactory for TestAppConfigFactory {
    fn get_config(&self) -> AppConfig {
        AppConfig {
            database_url: self.config.database_url.to_owned(),
            service_port: self.config.port,
        }
    }
}

pub(crate) async fn start_server(config: TestAppConfig) -> std::io::Result<Server> {
    crate::http::server::start(TestAppConfigFactory { config }).await
}
