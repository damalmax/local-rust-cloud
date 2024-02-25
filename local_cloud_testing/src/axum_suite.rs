use std::future::Future;
use std::io::Error;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::Router;
use parking_lot::ReentrantMutex;
use tokio::net::TcpListener;

pub struct AxumTestSuite {
    lock: ReentrantMutex<()>,
}

impl AxumTestSuite {
    pub const fn new() -> Self {
        AxumTestSuite {
            lock: ReentrantMutex::new(()),
        }
    }

    pub async fn create_test_ctx<F>(&self, create_router_fn: impl Fn(u16) -> F) -> AxumTestContext
    where
        F: Future<Output = std::io::Result<Router>>,
    {
        let guard = self.lock.lock();
        loop {
            let port = local_cloud_common::network::get_available_port();
            match AxumTestContext::start_new(port, create_router_fn(port).await.unwrap()).await {
                Ok(test_context) => {
                    drop(guard);
                    return test_context;
                }
                Err(error) => log::error!("{}", error),
            }
        }
    }
}

pub async fn create_test_ctx<F>(create_router_fn: impl Fn(u16) -> F) -> AxumTestContext
where
    F: Future<Output = std::io::Result<Router>>,
{
    AxumTestSuite::new().create_test_ctx(create_router_fn).await
}

#[derive(Debug)]
pub struct AxumTestContext {
    pub port: u16,
    server_event_sender: tokio::sync::oneshot::Sender<ServerEvent>,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ServerEvent {
    Terminate,
}

impl AxumTestContext {
    pub fn new(port: u16, server_event_sender: tokio::sync::oneshot::Sender<ServerEvent>) -> Self {
        AxumTestContext {
            port,
            server_event_sender,
        }
    }

    pub async fn start_new(port: u16, router: Router) -> Result<Self, Error> {
        // start HTTP server
        let listener = TcpListener::bind(("0.0.0.0", port)).await?;

        let (server_event_sender, mut rx) = tokio::sync::oneshot::channel();

        let test_start_time_millis = current_time_millis();

        tokio::spawn(async move {
            // Run the server with graceful shutdown
            axum::serve(listener, router)
                .with_graceful_shutdown(async move {
                    loop {
                        // wait few milliseconds before next check to avoid high CPU usage
                        tokio::time::sleep(Duration::from_millis(100)).await;

                        if (current_time_millis() - test_start_time_millis) > 30000 {
                            // test execution timeout (default 30 seconds)
                            break;
                        }
                        match rx.try_recv() {
                            Ok(server_event) => {
                                if server_event == ServerEvent::Terminate {
                                    break;
                                }
                            }
                            Err(_) => {
                                continue;
                            }
                        }
                    }
                })
                .await
                .unwrap();
        });

        Ok(AxumTestContext::new(port, server_event_sender))
    }

    pub async fn stop_server(self) {
        self.server_event_sender.send(ServerEvent::Terminate).unwrap();
    }
}

fn current_time_millis() -> u128 {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis()
}
