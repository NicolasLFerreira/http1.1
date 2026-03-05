mod parser;
mod tcp_client;
mod tcp_server;
mod types;
mod utils;

use std::env;

fn main() {
    let request = "\
    GET /path HTTP/1.1\r\n\
    Host: example.com\r\n\
    Connection: keep-alive\r\n\
    Content-Length: 3\r\n\
    \r\n\
    Abc";

    if env::args().into_iter().len() > 1 {
        println!("Launching client");
        let client = tcp_client::Client::new(8080);
        client.start();
    } else {
        println!("Launching server");
        let server = tcp_server::Server::new(8080);
        server.start();
    }
}
