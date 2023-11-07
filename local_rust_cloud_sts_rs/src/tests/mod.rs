use actix_web::dev::ServerHandle;
use aws_credential_types::provider::ProvideCredentials;
use std::net::TcpListener;

#[cfg(test)]
mod assume_role;

#[derive(Debug)]
pub struct TestContext {
    pub port: u16,
    pub server_handle: ServerHandle,
}

impl TestContext {
    pub async fn new() -> TestContext {
        let port = get_available_port().expect("Failed to bind available port for Test Server");

        let server = crate::create_http_server(|| crate::config::AppConfig::with_params("file::memory:?cache=shared", port.clone()))
            .await
            .expect("Failed to start Test Server");

        let server_handle = server.handle();
        actix_rt::spawn(server);
        TestContext { port, server_handle }
    }

    pub async fn stop_server(&mut self) {
        self.server_handle.stop(false).await;
    }
}

pub fn get_available_port() -> Option<u16> {
    return (4500..4600).find(|port| is_port_available(*port));
}

fn is_port_available(port: u16) -> bool {
    return match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    };
}

pub fn credentials_provider() -> impl ProvideCredentials {
    aws_credential_types::Credentials::new("access_key_id", "secret_access_key", Option::None, Option::None, "provider_name")
}
