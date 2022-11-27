use std::collections::HashMap;

use super::status_codes::StatusCodes;

pub fn parse_headers(headers_string: &str) -> Result<HashMap<String, String>, StatusCodes> {
    let header_lines = headers_string.split("\r\n");
    let mut headers: HashMap<String, String> = HashMap::new();

    for line in header_lines {
        let Some((k, v)) = line.split_once(':') else {
            println!("Error(parse_headers): incorrect string format.\n{}\n", line);
            return Err(StatusCodes::BadRequest);
        };

        headers.insert(k.trim().to_lowercase().to_owned(), v.trim().to_owned());
    }

    Ok(headers)
}
