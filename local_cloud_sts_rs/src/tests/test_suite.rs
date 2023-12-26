use std::{io::Error, net::TcpListener};

use actix_web::dev::ServerHandle;
use parking_lot::ReentrantMutex;
use uuid::Uuid;

pub struct TestSuite {
    lock: ReentrantMutex<()>,
}

impl TestSuite {
    pub const fn new() -> TestSuite {
        TestSuite {
            lock: ReentrantMutex::new(()),
        }
    }

    pub async fn create_test_ctx(&self) -> TestContext {
        let guard = self.lock.lock();
        loop {
            match TestContext::new().await {
                Ok(test_context) => {
                    drop(guard);
                    return test_context;
                }
                Err(error) => log::error!("{}", error),
            }
        }
    }
}

#[derive(Debug)]
pub struct TestContext {
    pub port: u16,
    pub server_handle: ServerHandle,
}

impl TestContext {
    pub async fn new() -> Result<TestContext, Error> {
        let port = get_available_port().expect("Failed to bind available port for Test Server");

        let db_file_name = Uuid::new_v4();
        let server = crate::create_http_server(|| {
            crate::config::AppConfig::with_params(format!("file:{}?mode=memory&cache=shared", db_file_name), port.clone())
        })
        .await;

        match server {
            Ok(server_handler) => {
                let server_handle = server_handler.handle();
                actix_rt::spawn(server_handler);

                return Result::Ok(TestContext { port, server_handle });
            }
            Err(e) => return Result::Err(Error::new(e.kind(), e)),
        }
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
