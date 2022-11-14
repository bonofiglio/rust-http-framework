use std::collections::HashMap;

use super::{message, method::HTTPMethod, status_codes::StatusCodes};

#[derive(Debug)]
pub struct Request<'a> {
    method: HTTPMethod,
    uri: &'a str,
    headers: HashMap<&'a str, &'a str>,
    body: &'a str,
}

impl<'a> Request<'a> {
    pub fn new(
        method: HTTPMethod,
        uri: &'a str,
        headers: HashMap<&'a str, &'a str>,
        body: &'a str,
    ) -> Request<'a> {
        Request {
            method,
            uri,
            headers,
            body,
        }
    }

    pub fn from(request_string: &str) -> Result<Request, StatusCodes> {
        let result = Request::parse_string(request_string);

        match result {
            Some(result) => Ok(Request {
                method: result.0,
                uri: result.1,
                headers: result.2,
                body: result.3,
            }),
            None => Err(StatusCodes::BadRequest),
        }
    }

    fn parse_request_line(request_line: &str) -> Option<(HTTPMethod, &str)> {
        let (method, rest) = request_line.split_once(' ')?;
        // TODO: Might wanna regex test the uri to check its validity
        let (uri, _) = rest.split_once(' ')?;

        let parsed_method = match method {
            "OPTIONS" => Some(HTTPMethod::OPTIONS),
            "GET" => Some(HTTPMethod::GET),
            "HEAD" => Some(HTTPMethod::HEAD),
            "POST" => Some(HTTPMethod::POST),
            "PUT" => Some(HTTPMethod::PUT),
            "DELETE" => Some(HTTPMethod::DELETE),
            "TRACE" => Some(HTTPMethod::TRACE),
            "CONNECT" => Some(HTTPMethod::CONNECT),
            _ => None,
        }?;

        Some((parsed_method, uri))
    }

    fn get_sections(request_string: &str) -> Option<(&str, &str, &str)> {
        let (request_line, rest) = request_string.split_once("\r\n")?;
        let (headers, body) = rest.split_once("\r\n\r\n")?;

        Some((request_line, headers, body))
    }

    fn parse_string(request_string: &str) -> Option<(HTTPMethod, &str, HashMap<&str, &str>, &str)> {
        let (request_line, headers_string, body) = Request::get_sections(request_string)?;

        let (method, uri) = Request::parse_request_line(request_line)?;
        let headers = message::parse_headers(headers_string)?;

        Some((method, uri, headers, body))
    }
}
