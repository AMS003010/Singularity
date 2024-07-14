// TODO: Check for one port and switch to another if busy





// use std::net::{TcpListener, TcpStream};

// pub fn find_available_port(start_port: u16) -> u16 {
//     let mut port = start_port;
//     loop {
//         if is_port_ok(port) {
//             return port;
//         }
//         println!("Looks like port {} is busy ⛔",port);
//         port+=1;
//     }
// }

// fn is_port_ok(port: u16) -> bool {
//     TcpListener::bind("127.0.0.1",port).is_ok()
// }

// fn main(){
//     let start_port = 5500;
//     let a = find_available_port(start_port);
//     println!("Switching to port {} ⚓",a);
// }