use async_std::{
    io::{prelude::BufReadExt, BufReader, ReadExt},
    net::{TcpListener, TcpStream},
    stream::StreamExt,
    task,
};
use routes::{Route, RouteHandler};
use std::{collections::HashMap, io::Error};

use http_types::{
    message, method::HTTPMethod, request::Request, response::Response, status_codes::StatusCodes,
};

pub struct Server {
    address: String,
    port: String,
    listener: Option<TcpListener>,
    handlers: HashMap<HTTPMethod, Vec<RouteHandler>>,
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
                    println!("{}", e);
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
        let headers = message::parse_headers(headers_string)?;

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

        Ok(Request::new(method, uri, headers, body))
    }

    pub fn new(port: &str) -> Result<Server, Error> {
        Ok(Server {
            address: "127.0.0.1".to_owned(),
            port: port.to_owned(),
            listener: None,
            handlers: Server::create_empty_handlers(),
        })
    }

    pub fn with_address(address: &str, port: &str) -> Result<Server, Error> {
        Ok(Server {
            address: address.to_owned(),
            port: port.to_owned(),
            listener: None,
            handlers: Server::create_empty_handlers(),
        })
    }

    // Private functions
    async fn handle_connection(
        mut stream: TcpStream,
        handlers: HashMap<HTTPMethod, Vec<fn(&Request) -> Response>>,
    ) {
        let request = Server::decode_request(&stream).await;

        match request {
            Ok(request) => {
                let handlers_vector = handlers.get(&request.method);

                match handlers_vector {
                    Some(handlers_vector) => {
                        for handler in handlers_vector {
                            let response = handler(&request);

                            response.send(&mut stream).await;
                        }
                    }
                    None => {}
                }
            }
            Err(status) => {
                let response = Response::new(status, HashMap::new(), "");

                response.send(&mut stream).await;
            }
        };
    }

    fn create_empty_handlers() -> HashMap<HTTPMethod, Vec<fn(&Request) -> Response>> {
        HashMap::from([
            (HTTPMethod::OPTIONS, Vec::new()),
            (HTTPMethod::GET, Vec::new()),
            (HTTPMethod::HEAD, Vec::new()),
            (HTTPMethod::POST, Vec::new()),
            (HTTPMethod::PUT, Vec::new()),
            (HTTPMethod::DELETE, Vec::new()),
            (HTTPMethod::TRACE, Vec::new()),
            (HTTPMethod::CONNECT, Vec::new()),
        ])
    }

    // Private methods
    fn get_handlers_vector(&mut self, method: &HTTPMethod) -> &mut Vec<fn(&Request) -> Response> {
        let result = self.handlers.get_mut(method);

        match result {
            Some(handlers_vector) => handlers_vector,
            None => panic!("Could not find handlers vector"),
        }
    }

    async fn _init(&mut self) -> Result<(), Error> {
        self.listener = Some(TcpListener::bind(format!("{}:{}", self.address, self.port)).await?);

        self.listen().await;

        Ok(())
    }

    // Public methods
    pub fn init(&mut self) {
        task::block_on(self._init());
    }

    pub fn add_routes(&mut self, routes: Vec<Route>) {
        for route in routes {
            self.add_handler(route.method, route.handler)
        }
    }

    pub fn add_handler(&mut self, method: HTTPMethod, handler: fn(&Request) -> Response) {
        let handlers_vector = Server::get_handlers_vector(self, &method);

        handlers_vector.push(handler);
    }

    async fn listen(&self) {
        let mut incoming = self.listener.as_ref().unwrap().incoming();

        while let Some(stream) = incoming.next().await {
            match stream {
                Ok(stream) => {
                    let handlers_clone = self.handlers.clone();
                    task::spawn(Server::handle_connection(stream, handlers_clone));
                }
                Err(error) => {
                    println!("{}", error);
                }
            };
        }
    }
}
