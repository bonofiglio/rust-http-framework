use std::collections::HashMap;

pub fn parse_headers(headers_string: &str) -> Option<HashMap<&str, &str>> {
    let header_lines = headers_string.split("\r\n");
    let mut headers: HashMap<&str, &str> = HashMap::new();

    for line in header_lines {
        let (k, v) = line.split_once(':')?;

        headers.insert(k.trim(), v.trim());
    }

    Some(headers)
}
