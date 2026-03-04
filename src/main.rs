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
    \r\n\
    This is something\r\n\
    blah blah blah\r\n";

    if env::args().into_iter().len() > 1 {
        println!("Launching client");
        let client = tcp_client::Client::new(8080);
        client.start();
    } else {
        println!("Launching server");
        let server = tcp_server::Server::new(8080);
        server.start();
    }
    //
    // let bytes: Vec<u8> = str_to_bytes(request);
    //
    // dbg!(&bytes);
    // dbg!(parse_bytes(bytes));
}

// let mut args = env::args();
// let _program = args.next();
//
// let launch: LaunchType;
// if let Some(a) = args.next()
// && a.as_str().eq("client")
// {
// println!("Launching as client");
// launch = LaunchType::Client;
// } else {
// println!("Launching as server");
// launch = LaunchType::Server;
// }
//
// match launch {
// LaunchType::Server => server(),
// LaunchType::Client => client(),
// }
