use std::{collections::HashMap, str::Split};

#[derive(Debug, Clone)]
pub struct UriParser {
    pub path: String,
    pub dynamic_params: HashMap<usize, String>,
}

impl UriParser {
    pub fn parse(&self, uri: &str) -> Result<Vec<(String, String)>, String> {
        let uri = UriParser::extract_fragment(uri);
        let (path, _) = UriParser::split_search(uri);
        let mut vec = Vec::with_capacity(self.dynamic_params.len());

        let mut idx = 0;
        for value in path.split('/') {
            match self.dynamic_params.get(&idx) {
                Some(key) => vec.push((key.to_owned(), value.to_owned())),
                None => {}
            }

            idx += 1;
        }

        if vec.len() != self.dynamic_params.len() {
            return Err(format!(
                "Uri does not match expected format. Expected {} parameters, got {}. Path: {}. Dynamic params: {:#?}",
                self.dynamic_params.len(),
                vec.len(),
                path,
                self.dynamic_params
            ));
        }

        Ok(vec)
    }

    pub fn matches(&self, uri: &str) -> bool {
        let uri = UriParser::extract_fragment(uri);
        let (path, _) = UriParser::split_search(uri);
        let split_path = path.split('/');
        let parser_split_path_vec: Vec<_> = self.path.split('/').collect();
        let parser_param_count = parser_split_path_vec.len();
        let param_count = split_path.count();

        // Check that the param count matches with the one in the UriParser
        if param_count != parser_param_count {
            return false;
        }

        // Iterate over the params
        let mut idx = 0;
        for value in path.split('/') {
            // Check if the uri has a slug in the place of that param
            let parser_param = self.dynamic_params.get(&idx);

            if parser_param.is_some() {
                // If it does, continue to the next one
                idx += 1;
                continue;
            }

            // If it doesn't, check if the param from the uri matches the one from the UriParser
            let parser_value = parser_split_path_vec[idx];

            if parser_value != value {
                // If it doesn't, return false
                return false;
            }

            idx += 1;
        }

        // If all params match, return true
        return true;
    }

    pub fn from(uri: &str) -> UriParser {
        let uri = UriParser::extract_fragment(uri);
        let (path, _) = UriParser::split_search(uri);

        let path_split = path.split("/");
        let dynamic_params = UriParser::get_dynamic_params(path_split);

        UriParser {
            path: path.to_owned(),
            dynamic_params: dynamic_params,
        }
    }

    pub fn extract_fragment(uri: &str) -> &str {
        match uri.split_once('#') {
            Some((uri_rest, _fragment)) => uri_rest,
            None => uri,
        }
    }

    pub fn split_search(uri: &str) -> (&str, &str) {
        match uri.split_once('?') {
            Some((path, search_string)) => (path, search_string),
            None => (uri, ""),
        }
    }

    fn get_dynamic_params(path_split: Split<&str>) -> HashMap<usize, String> {
        let mut params = HashMap::<usize, String>::with_capacity(path_split.clone().count());

        let mut idx = 0;
        for slice in path_split {
            if slice.starts_with(":") {
                let mut param = slice.to_owned();

                param.remove(0);

                params.insert(idx, param);
            }
            idx += 1;
        }

        params
    }
}
