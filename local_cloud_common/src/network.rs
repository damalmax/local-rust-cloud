use std::net::TcpListener;

pub fn get_available_port() -> u16 {
    loop {
        if let Some(port) = (4500..4600).find(|port| is_port_available(*port)) {
            return port;
        }
    }
}

fn is_port_available(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_ok()
}
