use utils::config;

const ENV_DATABASE_URL: &str = "STS_DATABASE_URL";
const ENV_SERVICE_PORT: &str = "STS_SERVICE_PORT";

const DEFAULT_DATABASE_URL: &str = "sqlite://sts.db";
const DEFAULT_SERVICE_PORT: u16 = 4502;

#[derive(Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub service_port: u16,
}

impl AppConfig {
    pub fn parse_env() -> Self {
        log::info!("Reading STS configurations...");

        let database_url = config::get_string_env_with_default(ENV_DATABASE_URL, DEFAULT_DATABASE_URL).into();
        let service_port = config::get_u16_env_with_default(ENV_SERVICE_PORT, DEFAULT_SERVICE_PORT);
        AppConfig {
            database_url,
            service_port,
        }
    }

    #[allow(dead_code)]
    pub fn with_params(database_url: impl Into<String>, port: u16) -> Self {
        AppConfig {
            database_url: database_url.into(),
            service_port: port,
        }
    }
}
