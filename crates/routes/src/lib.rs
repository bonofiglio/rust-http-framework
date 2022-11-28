use http_types::{method::HTTPMethod, request::Request, response::Response, uri_parser::UriParser};

pub type RouteHandler = fn(&Request) -> Response;

#[derive(Clone)]
pub struct Route {
    pub handler: RouteHandler,
    pub method: HTTPMethod,
    pub uri_parser: UriParser,
}

#[macro_export]
macro_rules! generate_routes {
    ($($e:ident),*) => {&mut {
        let mut v = Vec::<Route>::new();

        $(v.push($e::route());)*

        v
    }};
}
