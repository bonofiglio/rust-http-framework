#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
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
    pub fn from(method_string: &str) -> Option<HTTPMethod> {
        match method_string {
            "OPTIONS" => Some(HTTPMethod::OPTIONS),
            "GET" => Some(HTTPMethod::GET),
            "HEAD" => Some(HTTPMethod::HEAD),
            "POST" => Some(HTTPMethod::POST),
            "PUT" => Some(HTTPMethod::PUT),
            "DELETE" => Some(HTTPMethod::DELETE),
            "TRACE" => Some(HTTPMethod::TRACE),
            "CONNECT" => Some(HTTPMethod::CONNECT),
            _ => None,
        }
    }

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
