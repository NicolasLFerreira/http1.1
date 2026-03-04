use std::collections::HashMap;

#[derive(Default, Debug)]
pub enum HttpMethod {
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
pub struct HttpRequestLine {
    pub method: HttpMethod,
    pub path: String,
    pub version: String,
}

#[derive(Default, Debug)]
pub struct HttpRequest {
    pub request_line: HttpRequestLine,
    pub host: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}
