mod lib;
use std::collections::HashMap;

use http_types::method::HTTPMethod;
use http_types::request::Request;
use http_types::response::Response;
use http_types::status_codes::StatusCodes;
use http_types::uri_parser::UriParser;
use route_attribute_macro::route;
use routes::generate_routes;
use routes::Route;

use lib::server::Server;

#[route("GET", "/:p1")]
fn get_0(req: &Request) -> Response {
    Response::new(
        StatusCodes::OK,
        HashMap::new(),
        &format!("0: {}, search: {:#?}", &p1, req.search_params),
    )
}

#[route("GET", "/users/:slug")]
fn get_1(req: &Request) -> Response {
    Response::new(StatusCodes::OK, HashMap::new(), &format!("3: {}", &slug))
}

#[route("GET", "/accounts/:a/addresses/:b")]
fn get_2(req: &Request) -> Response {
    Response::new(
        StatusCodes::OK,
        HashMap::new(),
        &format!("4: {} {}", &a, &b),
    )
}

fn main() {
    let mut server = Server::new("3000").unwrap();

    server.add_routes(generate_routes![get_0, get_1, get_2]);

    server.init();
}
