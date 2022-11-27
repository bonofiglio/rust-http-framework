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
        let content_length = body.as_bytes().len().to_string();
        headers.entry("content-length".to_owned()).and_modify(|e| *e = content_length.to_owned()).or_insert(content_length);
        
        if !headers.contains_key("content-type") {
            headers.insert("content-type".to_owned(), "text/plain".to_owned());
        }

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
