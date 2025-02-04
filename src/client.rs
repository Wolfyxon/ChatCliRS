use std::{io::Read, net::TcpStream};

pub fn join(address: String) {
    let client = TcpStream::connect(address);

    match client {
        Ok(mut client) => {
            loop {
                let mut buf: [u8; 256] = [0; 256];
                
                match client.read(&mut buf) {
                    Ok(_) => {
                        println!("{:?}", buf);
                    }
                    
                    Err(_) => {}
                }
            }
        }
        Err(err) => {
            println!("Failed to connect: {err}");
        }
    }
}