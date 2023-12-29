use std::net::TcpListener;

pub fn get_available_port() -> Option<u16> {
    return (4500..4600).find(|port| is_port_available(*port));
}

fn is_port_available(port: u16) -> bool {
    return match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    };
}
