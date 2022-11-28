use std::collections::HashMap;

use crate::search_params::SearchParams;

use super::{method::HTTPMethod, status_codes::StatusCodes};

#[derive(Debug)]
pub struct Request {
    pub method: HTTPMethod,
    pub uri: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub search_params: SearchParams,
}

impl Request {
    pub fn new(
        method: HTTPMethod,
        uri: String,
        headers: HashMap<String, String>,
        body: String,
        search_params: SearchParams,
    ) -> Request {
        Request {
            method,
            uri,
            headers,
            body,
            search_params,
        }
    }

    pub fn parse_request_line(request_line: &str) -> Result<(HTTPMethod, String), StatusCodes> {
        let Some((method, rest)) = request_line.split_once(' ') else {
            println!("Error(parse_request_line[0]): incorrect string format.\n{}\n", request_line);
            return Err(StatusCodes::BadRequest);
        };

        // TODO: Might wanna regex test the uri to check its validity
        let Some((uri, _)) = rest.split_once(' ') else {
            println!("Error(parse_request_line[1]): incorrect string format.\n{}\n", request_line);
            return Err(StatusCodes::BadRequest);
        };

        let parsed_method = match method {
            "OPTIONS" => HTTPMethod::OPTIONS,
            "GET" => HTTPMethod::GET,
            "HEAD" => HTTPMethod::HEAD,
            "POST" => HTTPMethod::POST,
            "PUT" => HTTPMethod::PUT,
            "DELETE" => HTTPMethod::DELETE,
            "TRACE" => HTTPMethod::TRACE,
            "CONNECT" => HTTPMethod::CONNECT,
            _ => return Err(StatusCodes::MethodNotAllowed),
        };

        Ok((parsed_method, uri.to_owned()))
    }

    pub fn get_sections(request_string: &str) -> Result<(&str, &str, &str), StatusCodes> {
        let Some((request_line, rest)) = request_string.split_once("\r\n") else {
            println!("Error(get_sections): could not get request_line.\n{}\n", request_string);
            return Err(StatusCodes::BadRequest);
        };
        let Some((headers, body)) = rest.split_once("\r\n\r\n") else {
            println!("Error(get_sections): could not split headers and body.\n{}\n", request_string);
            return Err(StatusCodes::BadRequest);
        };

        Ok((request_line, headers, body))
    }
}
