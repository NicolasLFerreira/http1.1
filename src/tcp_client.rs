use crate::utils::str_to_bytes;
use std::io::Write;
use std::net::TcpStream;

pub struct Client {
    port: u16,
}

impl Client {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub fn start(self) {
        let request = "\
        GET /path HTTP/1.1\r\n\
        Host: example.com\r\n\
        Connection: keep-alive\r\n\
        Content-Length: 3\r\n\
        \r\n\
        Abc";

        let bytes = str_to_bytes(request);

        let address = format!("127.0.0.1:{}", self.port);
        let mut stream = TcpStream::connect(address).expect("Failed to connect.");

        println!("{}", bytes.iter().len());

        if let Err(e) = stream.write_all(&bytes[..]) {
            println!("Failed to write stream: {e:?}")
        }
    }
}
