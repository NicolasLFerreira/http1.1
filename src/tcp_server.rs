use crate::parser::parse_bytes;
use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::ops::Deref;
use std::thread;

pub struct Server {
    port: u16,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub fn start(self) {
        let address = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(address).expect("Could not establish bind.");

        loop {
            println!("Awaiting connection...");
            match listener.accept() {
                Ok((stream, addr)) => {
                    println!("Established connection: {}", addr);
                    let handle = thread::spawn(move || {
                        handle_connection(stream, addr);
                    });
                    handle.join().expect("Failed to join threads");
                }
                Err(e) => {
                    println!("Failed to establish connection: {e:?}")
                }
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, addr: SocketAddr) {
    let mut buffer: Vec<u8> = Vec::new();
    let mut temp = [0u8; 1024];

    loop {
        let n = stream.read(&mut temp).unwrap();
        if n == 0 {
            break;
        }

        buffer.extend_from_slice(&temp[..n]);
    }

    let http_request = parse_bytes(buffer);
    dbg!(http_request);
}
