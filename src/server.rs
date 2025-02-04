use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

pub fn host(port: u16) {
    let server = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port)).unwrap();

    for stream in server.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection");
            }

            Err(_) => {}
        }
    }
}