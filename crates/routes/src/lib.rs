use http_types::{method::HTTPMethod, request::Request, response::Response};

pub type RouteHandler = fn(&Request) -> Response;

pub struct Route {
    pub handler: RouteHandler,
    pub method: HTTPMethod,
    pub path: String,
}

#[macro_export]
macro_rules! generate_routes {
    ($($e:ident),*) => {{
        let mut v = Vec::<Route>::new();

        $(v.push($e::route());)*

        v
    }};
}
