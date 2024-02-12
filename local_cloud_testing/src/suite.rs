use std::future::Future;
use std::io::Error;

use actix_web::dev::{Server, ServerHandle};
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

    pub async fn create_test_ctx<F>(&self, start_server_fn: impl Fn(TestAppConfig) -> F) -> TestContext
    where
        F: Future<Output = std::io::Result<Server>>,
    {
        let guard = self.lock.lock();
        loop {
            match TestContext::new(&start_server_fn).await {
                Ok(test_context) => {
                    drop(guard);
                    return test_context;
                }
                Err(error) => log::error!("{}", error),
            }
        }
    }
}

pub async fn create_test_ctx<F>(start_server_fn: impl Fn(TestAppConfig) -> F) -> TestContext
where
    F: Future<Output = std::io::Result<Server>>,
{
    TestSuite::new().create_test_ctx(start_server_fn).await
}

#[derive(Debug)]
pub struct TestContext {
    pub port: u16,
    pub server_handle: ServerHandle,
}

#[derive(Debug)]
pub struct TestAppConfig {
    pub database_url: String,
    pub port: u16,
}

impl TestContext {
    pub async fn new<F>(start_server_fn: impl Fn(TestAppConfig) -> F) -> Result<TestContext, Error>
    where
        F: Future<Output = std::io::Result<Server>>,
    {
        let port = local_cloud_common::network::get_available_port();

        let db_file_name = Uuid::new_v4();
        let server_config = TestAppConfig {
            // database_url: "sqlite://iam-test.db".to_owned(),
            database_url: format!("file:{}?mode=memory&cache=shared", db_file_name),
            port,
        };
        let server = start_server_fn(server_config).await;

        match server {
            Ok(server_handler) => {
                let server_handle = server_handler.handle();
                actix_rt::spawn(server_handler);

                Ok(TestContext { port, server_handle })
            }
            Err(e) => Err(Error::new(e.kind(), e)),
        }
    }

    pub async fn stop_server(&mut self) {
        self.server_handle.stop(false).await;
    }
}
