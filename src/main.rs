use std::collections::HashMap;

fn main() {
    let sep: [u8; 4] = [0x0D, 0x0A, 0x0D, 0x0A];

    let request = "\
    GET /path HTTP/1.1\n\
    Host: example.com\n\
    Connection: keep-alive\n\
    \n\
    This is something\n\
    blah blah blah";

    dbg!(parser(request));
}

fn parser(request: &str) -> HttpRequest {
    let mut http_request = HttpRequest::default();

    let headers_end = request
        .find("\r\n\r\n")
        .or_else(|| request.find("\n\n"))
        .unwrap()
        + 4;

    http_request.body = request[headers_end..].to_string();

    let headers = &request[..headers_end];
    let mut lines = headers.lines();

    // request line
    if let Some(request_line) = lines.next() {
        let mut rl = HttpRequestLine::default();
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        match parts[0] {
            "GET" => rl.method = HttpMethod::GET,
            "POST" => rl.method = HttpMethod::POST,
            "PUT" => rl.method = HttpMethod::PUT,
            _ => { /* error handling */ }
        }
        rl.path = String::from(parts[1]);
        rl.version = String::from(parts[2]);

        http_request.request_line = rl;
    }

    // headers
    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() {
            // end of headers
            break;
        }

        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();
            match key {
                "Host" => http_request.host = value.to_string(),
                _ => {
                    http_request
                        .headers
                        .insert(key.to_string(), value.to_string());
                }
            }
        }
    }

    http_request
}

#[derive(Default, Debug)]
enum HttpMethod {
    #[default]
    GET,
    POST,
    PUT,
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
