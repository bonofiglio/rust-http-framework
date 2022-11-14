use async_std::{
    io::{prelude::BufReadExt, BufReader, WriteExt},
    net::{TcpListener, TcpStream},
    stream::StreamExt,
};
use std::{collections::HashMap, fmt::Write, io::Error, thread};

use super::{request::Request, response::Response};

pub struct Server<'a> {
    listener: TcpListener,
    handlers: Vec<&'a dyn Fn(&Request) -> Response<'a>>,
}

impl<'a> Server<'a> {
    pub async fn new(port: &'a str) -> Result<Server<'a>, Error> {
        println!("3");
        let listener = TcpListener::bind(format!("{}:{}", "127.0.0.1", port)).await?;

        println!("4");

        Ok(Server {
            listener,
            handlers: Vec::new(),
        })
    }

    pub async fn with_address(address: &'a str, port: &'a str) -> Result<Server<'a>, Error> {
        let listener = TcpListener::bind(address).await?;

        Ok(Server {
            listener,
            handlers: Vec::new(),
        })
    }

    pub fn add_handler(&mut self, handler: &'a dyn Fn(&Request) -> Response<'a>) {
        self.handlers.push(handler);
    }

    pub fn add_handlers(&mut self, handlers: Vec<&'a dyn Fn(&Request) -> Response<'a>>) {
        self.handlers.extend(&handlers);
    }

    async fn get_request_string(stream: &TcpStream) -> Result<String, Error> {
        let (reader, _) = (stream, stream);
        let mut lines = BufReader::new(reader).lines().fuse();
        let mut request_string = String::new();
        let mut i = 0;
        loop {
            if i == 8 {
                break;
            }
            let line = lines.next().await;

            match line {
                None => break,
                Some(line) => match line {
                    Err(error) => break,
                    Ok(line) => {
                        request_string.push_str(&line);
                        request_string.push_str("\r\n");
                    }
                },
            };
            i += 1;
        }

        Ok(request_string)
    }

    pub async fn listen(&self) {
        let mut incoming = self.listener.incoming();

        for stream in incoming.next().await {
            match stream {
                Ok(mut stream) => {
                    // thread::spawn(async || -> Result<(), Error> {
                    let request_string = Server::get_request_string(&stream).await;

                    match request_string {
                        Ok(request_string) => {
                            let request = Request::from(&request_string);

                            match request {
                                Ok(request) => {
                                    for handler in self.handlers.clone() {
                                        let response = handler(&request);

                                        response.send(&mut stream).await;
                                    }
                                }
                                Err(status) => {
                                    let response = Response::new(status, HashMap::new(), "");

                                    response.send(&mut stream).await;
                                }
                            };
                        }
                        Err(error) => {
                            println!("{}", error);
                        }
                    }

                    // Ok(())
                    // });
                }
                Err(error) => {}
            }
        }
    }
}
