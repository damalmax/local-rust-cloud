use local_cloud_common::config;

const ENV_DATABASE_URL: &str = "IAM_DATABASE_URL";
const ENV_ETCD_ENDPOINTS: &str = "ETCD_ENDPOINTS";
const ENV_ETCD_ENABLED: &str = "ETCD_ENABLED";
const ENV_SERVICE_PORT: &str = "IAM_SERVICE_PORT";

const DEFAULT_DATABASE_URL: &str = "sqlite://iam.db";
const DEFAULT_SERVICE_PORT: u16 = 4502;

#[derive(Debug)]
pub(crate) struct AppConfig {
    pub database_url: String,
    pub etcd_enabled: bool,
    pub etcd_endpoints: String,
    pub service_port: u16,
}

impl AppConfig {
    pub fn parse() -> Self {
        log::info!("Reading IAM configurations...");

        let database_url = config::get_string_env_with_default(ENV_DATABASE_URL, DEFAULT_DATABASE_URL).into();
        let etcd_enabled = config::get_bool_env_with_default(ENV_ETCD_ENABLED, false).into();
        let etcd_endpoints = config::get_string_env_with_default(ENV_ETCD_ENDPOINTS, "").into();
        let service_port = config::get_u16_env_with_default(ENV_SERVICE_PORT, DEFAULT_SERVICE_PORT);
        AppConfig {
            database_url,
            etcd_enabled,
            etcd_endpoints,
            service_port,
        }
    }

    #[allow(dead_code)]
    pub fn with_params(database_url: impl Into<String>, port: u16) -> Self {
        AppConfig {
            database_url: database_url.into(),
            etcd_enabled: false,
            etcd_endpoints: String::from(""),
            service_port: port,
        }
    }
}

pub(crate) trait AppConfigFactory {
    fn get_config(&self) -> AppConfig;
}

pub(crate) struct EnvironmentAppConfigFactory {}

impl EnvironmentAppConfigFactory {
    pub(crate) fn new() -> Self {
        EnvironmentAppConfigFactory {}
    }
}

impl AppConfigFactory for EnvironmentAppConfigFactory {
    fn get_config(&self) -> AppConfig {
        AppConfig::parse()
    }
}
