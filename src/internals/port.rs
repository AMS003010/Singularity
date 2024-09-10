use std::net::{TcpListener, TcpStream};

pub fn find_available_port(start_port: u16) -> u16 {
    let mut port = start_port;
    loop {
        println!("Trying to bind to port {}", port);
        if is_port_ok(port) {
            println!("Port {} is available ✔️", port);
            return port;
        }
        println!("Port {} is busy ⛔. Trying the next port...", port);
        port += 1;
    }
}

fn is_port_ok(port: u16) -> bool {
    let address = format!("127.0.0.1:{}", port);

    match TcpStream::connect(&address) {
        Ok(_) => {
            println!("Port {} is in use.", port);
            false
        }
        Err(_) => {
            match TcpListener::bind(&address) {
                Ok(_) => true, // Port is available
                Err(_) => false, // Port is unavailable
            }
        }
    }
}
