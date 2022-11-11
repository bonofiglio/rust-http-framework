mod lib;

use std::collections::HashMap;

use lib::message::Message;

fn main() {
    let message = Message::from(
        "POST HTTP/1.1\r\nAuthorization: Basic\r\nuser-agent: Chrome\r\n\r\n{\"name\": \"Name\"}",
    );

    println!("{:?}", message);
}
