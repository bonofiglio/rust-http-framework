use std::collections::HashMap;

use async_std::{io::WriteExt, net::TcpStream};

use super::status_codes::StatusCodes;

pub struct Response {
    status: StatusCodes,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    pub fn new(status: StatusCodes, mut headers: HashMap<String, String>, body: &str) -> Response {
        headers.insert(
            "Content-Length".to_owned(),
            body.as_bytes().len().to_string(),
        );

        headers.insert("Content-Type".to_owned(), "text/plain".to_owned());
        Response {
            status,
            headers,
            body: body.to_owned(),
        }
    }

    fn parse_headers(&self) -> String {
        let mut headers_string = String::new();
        for (key, value) in &self.headers {
            headers_string.push_str(&format!("{}: {}\r\n", *key, *value));
        }

        headers_string
    }

    pub fn to_string(&self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\n{}\r\n{}",
            self.status.as_code(),
            self.status.as_reason_phrase(),
            self.parse_headers(),
            self.body
        )
    }

    pub async fn send(&self, stream: &mut TcpStream) {
        let result = stream.write_all(self.to_string().as_bytes()).await;

        match result {
            Ok(_) => {}
            Err(error) => println!("{}", error),
        };
    }
}
