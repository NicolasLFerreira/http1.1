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
pub struct HttpHeader {
    pub method: HttpMethod,
    pub path: String,
    pub version: String,
    pub host: String,
    pub headers: HashMap<String, String>,
}

#[derive(Default, Debug)]
pub struct HttpBody {
    pub body: String
}

#[derive(Default, Debug)]
pub struct HttpRequest {
    pub header: HttpHeader,
    pub body: Option<HttpBody>
}
