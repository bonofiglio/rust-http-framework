use super::{method::HTTPMethod, status_codes::StatusCodes};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Message<'a> {
    method: HTTPMethod,
    headers: HashMap<&'a str, &'a str>,
    body: &'a str,
}

impl<'a> Message<'a> {
    pub fn new(
        method: HTTPMethod,
        headers: HashMap<&'a str, &'a str>,
        body: &'a str,
    ) -> Message<'a> {
        Message {
            method,
            headers,
            body,
        }
    }

    pub fn from(message_string: &str) -> Result<Message, StatusCodes> {
        let result = Message::parse_message_string(message_string);

        match result {
            Some(result) => Ok(Message {
                method: result.0,
                headers: result.1,
                body: result.2,
            }),
            None => Err(StatusCodes::BadRequest),
        }
    }

    fn get_sections(message_string: &str) -> Option<(&str, &str, &str)> {
        let (first_line, rest) = message_string.split_once("\r\n")?;
        let (headers, body) = rest.split_once("\r\n\r\n")?;

        Some((first_line, headers, body))
    }

    fn get_method(first_line: &str) -> Option<HTTPMethod> {
        let (method, rest) = first_line.split_once(' ')?;

        match method {
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

    fn get_headers(headers_string: &str) -> Option<HashMap<&str, &str>> {
        let header_lines = headers_string.split("\r\n");
        let mut headers: HashMap<&str, &str> = HashMap::new();

        for line in header_lines {
            let (k, v) = line.split_once(':')?;

            headers.insert(k.trim(), v.trim());
        }

        Some(headers)
    }

    fn parse_message_string(
        message_string: &str,
    ) -> Option<(HTTPMethod, HashMap<&str, &str>, &str)> {
        let (first_line, headers_string, body) = Message::get_sections(message_string)?;

        let method = Message::get_method(first_line)?;
        let headers = Message::get_headers(headers_string)?;

        Some((method, headers, body))
    }
}
