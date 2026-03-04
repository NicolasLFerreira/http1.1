use std::collections::HashMap;
use std::str::from_utf8;

fn main() {
    let request = "\
    GET /path HTTP/1.1\r\n\
    Host: example.com\r\n\
    Connection: keep-alive\r\n\
    \r\n\
    This is something\r\n\
    blah blah blah\r\n";

    let bytes: Vec<u8> = str_to_bytes(request);

    dbg!(&bytes);
    dbg!(parser_bytes(bytes));
}

fn str_to_bytes(string: &str) -> Vec<u8> {
    string.chars().map(|c| c as u8).collect()
}

fn parser_bytes(bytes: Vec<u8>) -> HttpRequest {
    const SEPARATOR: &[u8; 4] = &[0x0D, 0x0A, 0x0D, 0x0A];
    let mut request = HttpRequest::default();

    if bytes.len() < 4 {
        panic!();
    }

    let mut separator_index = 0;
    let mut windows = bytes.windows(4);
    while let Some(window) = windows.next() {
        if window == SEPARATOR {
            break;
        }
        separator_index += 1;
    }

    let header_lines: Vec<&[u8]> = (&bytes[0..separator_index]).split(|c| *c == 0x0D).map(|l| &l[1..]).collect();
    let body = &bytes[separator_index + 4..];

    // Process request line
    {
        let request_line = str::from_utf8(header_lines[0]).unwrap();
        let parts: Vec<&str> = request_line.split(|c| c == ' ').collect();

        request.request_line = HttpRequestLine {
            method: HttpMethod::from_string(parts[0]),
            path: parts[1].to_string(),
            version: parts[2].to_string(),
        }
    }

    for line in &header_lines[1..] {
        let kv = {
            let kv = line.split(|x| *x == 0x3A).collect::<Vec<_>>();
            (
                str::from_utf8(kv[0]).unwrap(),
                str::from_utf8(&kv[1][1..]).unwrap().to_string(),
            )
        };

        if kv.0 == "Host" {
            request.host = kv.1;
        } else {
            request.headers.insert(kv.0.to_string(), kv.1);
        }
    }

    request.body = str::from_utf8(body).unwrap().to_string();

    request
}

#[derive(Default, Debug)]
enum HttpMethod {
    #[default]
    GET,
    POST,
    PUT,
    UNKNOWN,
}

impl HttpMethod {
    pub fn from_string(string: &str) -> Self {
        match string {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            _ => HttpMethod::UNKNOWN,
        }
    }
}

#[derive(Default, Debug)]
struct HttpRequestLine {
    method: HttpMethod,
    path: String,
    version: String,
}

#[derive(Default, Debug)]
struct HttpRequest {
    request_line: HttpRequestLine,
    host: String,
    headers: HashMap<String, String>,
    body: String,
}
