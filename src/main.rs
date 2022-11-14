mod lib;

use async_std::task;
use std::collections::HashMap;
use std::io::Error;

use lib::request::Request;
use lib::response::Response;
use lib::server::Server;

use crate::lib::status_codes::StatusCodes;

fn get<'a>(req: &Request) -> Response<'a> {
    println!("{:?}", *req);

    Response::new(
        StatusCodes::OK,
        HashMap::from([("Cache-Control", "no-cache")]),
        "hello",
    )
}

async fn init() -> Result<(), Error> {
    let mut server = Server::new("3000").await?;

    server.add_handler(&get);

    server.listen().await;

    Ok(())
}

fn main() {
    // let message = Request::from(
    //     "POST HTTP/1.1\r\nAuthorization: Basic\r\nuser-agent: Chrome\r\n\r\n{\"name\": \"Name\"}",
    // );

    println!("1");

    task::block_on(init());
}
