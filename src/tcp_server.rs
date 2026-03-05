use crate::parser::{body_parser, header_parser};
use crate::types::{HttpBody, HttpHeader, HttpRequest};
use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};
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
        let listener = TcpListener::bind(address).unwrap();

        loop {
            println!("Awaiting connection...");
            match listener.accept() {
                Ok((stream, addr)) => {
                    println!("Established connection: {}", addr);
                    let handle = thread::spawn(move || {
                        handle_connection(stream, addr);
                    });
                    handle.join().unwrap();
                }
                Err(e) => {
                    eprintln!("Failed to establish connection: {e:?}")
                }
            }
        }
    }
}

fn handle_connection(stream: TcpStream, addr: SocketAddr) {
    let request = handle_parsing(stream).unwrap();
    dbg!(addr);
    dbg!(request.header);
    dbg!(request.body);
}

fn handle_parsing(mut stream: TcpStream) -> Result<HttpRequest, String> {
    let header: HttpHeader;
    let mut body: Option<HttpBody> = None;

    let mut buffer: Vec<u8> = Vec::new();
    let mut temp = [0u8; 1024];

    // Handle header
    loop {
        let n = stream.read(&mut temp).unwrap();
        if n == 0 {
            return Err("Failed to read header".to_string());
        }
        buffer.extend_from_slice(&temp[..n]);
        if let Some(header_end) = buffer
            .windows(4)
            .position(|x| x == [0x0D, 0x0A, 0x0D, 0x0A])
        {
            header = header_parser(&buffer[..header_end])?;
            let body_start = header_end + 4;

            buffer = buffer[body_start..].to_vec();
            break;
        }
    }

    let (c_length, remaining): (usize, usize) = if let Some(c_length) = header.headers.get("Content-Length") {
        let c_length = c_length.parse().unwrap();
        (c_length, buffer.len().saturating_sub(c_length))
    } else {
        (0, 0)
    };

    if remaining > 0 {
        let current = buffer.len();
        buffer.resize(c_length, 0);
        stream.read_exact(&mut buffer[current..]).unwrap();
    }

    body = Some(body_parser(&buffer)?);
    
    Ok(HttpRequest { header, body })
}
