mod lib;
use std::collections::HashMap;

use http_types::method::HTTPMethod;
use http_types::request::Request;
use http_types::response::Response;
use http_types::status_codes::StatusCodes;
use route_macro_attribute::route;
use routes::generate_routes;
use routes::Route;

use lib::server::Server;

#[route("GET", "/")]
fn get(req: &Request) -> Response {
    Response::new(StatusCodes::OK, HashMap::new(), "it works!")
}

#[route("POST", "/")]
fn post(req: &Request) -> Response {
    Response::new(StatusCodes::OK, HashMap::new(), &req.body)
}

fn main() {
    let mut server = Server::new("3000").unwrap();

    server.add_routes(generate_routes![get, post]);

    server.init();
}
