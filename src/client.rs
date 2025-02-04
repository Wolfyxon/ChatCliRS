use std::net::TcpStream;

pub fn join(address: String) {
    let client = TcpStream::connect(address);

    match client {
        Ok(mut client) => {
            println!("Connected");
        }
        Err(err) => {
            println!("Failed to connect: {err}");
        }
    }
}