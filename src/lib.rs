// Re-export the local crates as part of this library
pub use http_types::*;
pub use route_attribute_macro::*;
pub use routes::*;

use async_std::{
    io::{prelude::BufReadExt, BufReader, ReadExt},
    net::{TcpListener, TcpStream},
    stream::StreamExt,
    task,
};
use std::{collections::HashMap, io::Error, vec};

pub struct Server {
    address: String,
    port: String,
    listener: Option<TcpListener>,
    routes: Vec<Route>,
}

impl Server {
    // Public functions
    async fn decode_request(stream: &TcpStream) -> Result<Request, StatusCodes> {
        let mut reader = BufReader::new(stream);
        let mut buf = Vec::new();

        loop {
            let bytes_read = reader.read_until(b'\n', &mut buf).await;

            match bytes_read {
                Err(e) => {
                    println!("Error(decode_request): {}", e);
                    return Err(StatusCodes::BadRequest);
                }
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        // This means an abrupt ending to the stream.
                        return Err(StatusCodes::BadRequest);
                    }

                    let index = buf.len() - 1;
                    if index >= 3 && &buf[index - 3..=index] == b"\r\n\r\n" {
                        // Gracefully ended, break out of the loop
                        break;
                    }
                }
            }
        }

        let Ok(request_string) = String::from_utf8(buf) else {
            println!("Error(decode_request): could not parse buffer into String");
            return Err(StatusCodes::BadRequest);
        };

        let (request_line, headers_string, _) = Request::get_sections(&request_string)?;
        let headers = parse_headers(headers_string)?;

        let content_length_string = match headers.get("content-length") {
            Some(content_length) => content_length,
            None => "0",
        };

        let Ok(content_length) = content_length_string.parse::<usize>() else {
            println!("Error(decode_request): invalid content-length header format");
            return Err(StatusCodes::BadRequest);
        };

        let mut body_buffer = vec![0; content_length];

        let Ok(_) = reader.read_exact(&mut body_buffer).await else {
            println!("Error(decode_request): could not read body");
            return Err(StatusCodes::BadRequest);
        };

        let Ok(body) = String::from_utf8(body_buffer) else {
            println!("Error(decode_request): could not parse body to a string");
            return Err(StatusCodes::BadRequest);
        };
        let (method, uri) = Request::parse_request_line(request_line)?;
        let search_params = SearchParams::from(UriParser::split_search(&uri).1);

        Ok(Request::new(method, uri, headers, body, search_params))
    }

    pub fn new(port: &str) -> Result<Server, Error> {
        Ok(Server {
            address: "127.0.0.1".to_owned(),
            port: port.to_owned(),
            listener: None,
            routes: Vec::new(),
        })
    }

    pub fn with_address(address: &str, port: &str) -> Result<Server, Error> {
        Ok(Server {
            address: address.to_owned(),
            port: port.to_owned(),
            listener: None,
            routes: Vec::new(),
        })
    }

    // Private functions
    async fn handle_connection(mut stream: TcpStream, routes: Vec<Route>) {
        let request = Server::decode_request(&stream).await;

        match request {
            Ok(request) => {
                let route_handlers: Vec<_> = routes
                    .iter()
                    .filter(|route| route.uri_parser.matches(&request.uri))
                    .collect();

                if route_handlers.len() == 0 {
                    let response = Response::new(StatusCodes::NotFound, HashMap::new(), "");
                    response.send(&mut stream).await;
                    return;
                }

                for route in route_handlers {
                    let response = (route.handler)(&request);

                    response.send(&mut stream).await;
                }
            }
            Err(status) => {
                let response = Response::new(status, HashMap::new(), "");

                response.send(&mut stream).await;
            }
        };
    }

    // Private methods
    async fn _init(&mut self) -> Result<(), Error> {
        self.listener = Some(TcpListener::bind(format!("{}:{}", self.address, self.port)).await?);

        self.listen().await;

        Ok(())
    }

    // Public methods
    pub fn init(&mut self) {
        match task::block_on(self._init()) {
            Ok(_) => {}
            Err(e) => {
                println!("Error(init): {}", e);
            }
        }
    }

    pub fn add_routes(&mut self, routes: &mut Vec<Route>) {
        self.routes.append(routes)
    }

    async fn listen(&self) {
        let mut incoming = self.listener.as_ref().unwrap().incoming();

        while let Some(stream) = incoming.next().await {
            match stream {
                Ok(stream) => {
                    let handlers_clone = self.routes.clone();
                    task::spawn(Server::handle_connection(stream, handlers_clone));
                }
                Err(error) => {
                    println!("Error(listen): {}", error);
                }
            };
        }
    }
}
