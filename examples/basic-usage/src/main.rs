use rust_http_framework::*;
use std::collections::HashMap;

// Setting up a route using the "route" macro
#[route("GET", "/")]
fn get_root(req: &Request) -> Response {
    Response::new(StatusCodes::OK, HashMap::new(), "Hey!")
}

// Using dynamic routes
#[route("GET", "/users/:user_id")]
fn get_user(req: &Request) -> Response {
    // "user_id" is available as a variable in the context of this function
    Response::new(
        StatusCodes::OK,
        HashMap::new(),
        &format!("User: {{ \"id\": \"{}\" }}", &user_id),
    )
}

// Using the request object
#[route("POST", "/users")]
fn create_account(req: &Request) -> Response {
    // Check that the body is of type "application/json"
    match req.headers.get("content-type") {
        Some(content_type) => {
            if content_type.to_lowercase() != "application/json" {
                return Response::new(StatusCodes::UnsupportedMediaType, HashMap::new(), "");
            }
        }
        None => {
            return Response::new(StatusCodes::UnsupportedMediaType, HashMap::new(), "");
        }
    }

    let Ok(parsed_body) = json::parse(&req.body) else {
        return Response::new(StatusCodes::BadRequest, HashMap::new(), "");
    };

    // Expect "name" to be inside of the body
    let name = if parsed_body["name"].is_string() {
        parsed_body["name"].to_string()
    } else {
        return Response::new(
            StatusCodes::BadRequest,
            HashMap::new(),
            "\"name\" is required",
        );
    };

    Response::new(
        StatusCodes::OK,
        HashMap::new(),
        &format!("user {} created", &name),
    )
}

fn main() {
    // Create a server instance that listens on the port 3000
    let mut server = Server::new("3000").unwrap();

    // Add the routes and their handlers using the "generate_routes!" macro
    server.add_routes(generate_routes![get_root, get_user, create_account]);

    // Initialize the server
    server.init();
}
