#[derive(Debug)]
pub enum HTTPMethod {
    OPTIONS,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    TRACE,
    CONNECT,
}

impl HTTPMethod {
    pub fn as_str(&self) -> &str {
        match self {
            HTTPMethod::OPTIONS => "OPTIONS",
            HTTPMethod::GET => "GET",
            HTTPMethod::HEAD => "HEAD",
            HTTPMethod::POST => "POST",
            HTTPMethod::PUT => "PUT",
            HTTPMethod::DELETE => "DELETE",
            HTTPMethod::TRACE => "TRACE",
            HTTPMethod::CONNECT => "CONNECT",
        }
    }
}
