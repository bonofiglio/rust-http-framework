# Rust HTTP Framework

Asynchronous HTTP framework built from scratch using Rust.

- [Rust HTTP Framework](#rust-http-framework)
  - [Examples](#examples)
  - [Getting started](#getting-started)
    - [The `route` macro](#the-route-macro)
      - [Creating a handler](#creating-a-handler)
      - [Dynamic routes](#dynamic-routes)
    - [The `generate_routes` macro](#the-generate_routes-macro)
      - [Generating a list of routes from the handlers](#generating-a-list-of-routes-from-the-handlers)
    - [Creating a server](#creating-a-server)
      - [Creating an instance](#creating-an-instance)
      - [Adding routes to the instance](#adding-routes-to-the-instance)
      - [Starting the server](#starting-the-server)

## Examples

You can find real usage examples under the `examples/` folder

## Getting started

### The `route` macro

This macro is used to create a handler for a specific route. It takes two arguments: the HTTP method and the path.

#### Creating a handler

A handler is a function that takes a `Request` and returns a `Response`. The `Request` contains the request's method, path, headers and body. The `Response` contains the response's status code, headers and body. The `route` macro will automatically create the metadata for the handler and make it ready to be used by the server.

```rust
#[route("GET", "/")]
fn get_root(req: &Request) -> Response {
    Response::new(
        StatusCodes::OK,
        HashMap::new(),
        "Hello World!"
    )
}
```

#### Dynamic routes

You can also create dynamic routes by using the `:` prefix. The value of the dynamic route will be available as a variable in the handler.

```rust
#[route("GET", "/user/:user_id")]
fn get_user(req: &Request) -> Response {
    Response::new(
        StatusCodes::OK,
        HashMap::new(),
        format!("User with id {} was found!", user_id)
    )
}
```

### The `generate_routes` macro

This macro is used to generate a vector of routes from the handlers. It takes a list of handlers as arguments.

#### Generating a list of routes from the handlers

```rust
let routes = generate_routes!(
    get_root,
    get_user
);
```

### Creating a server

The server is the main component of the framework. It is responsible for listening to incoming requests and dispatching them to the appropriate handler.

#### Creating an instance

To create an instance of the server, you need to provide the port to listen to. You may also provide an address to listen to. If no address is provided, the server will listen the `localhost` address (`127.0.0.1`).

```rust
let server = Server::new("3000").unwrap();
```

#### Adding routes to the instance

To add routes to the server, you need to provide a list of routes to the `add_routes` method. You can easily generate a list of routes using [the `generate_routes` macro](#the-generate_routes-macro).

```rust
let routes = generate_routes!(
    get_root,
    get_user
);

server.add_routes(routes);
```

#### Starting the server

To start the server, you need to call the `start` method. This method will block the current thread and will listen to incoming requests.

```rust
server.start();
```
