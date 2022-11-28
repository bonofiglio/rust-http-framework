use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SearchParams {
    values: HashMap<String, Vec<String>>,
}

impl SearchParams {
    pub fn get(&self, key: &str) -> Option<&str> {
        match self.get_all(key) {
            None => None,
            Some(values) => Some(&values[0]),
        }
    }

    pub fn get_all(&self, key: &str) -> Option<&Vec<String>> {
        match self.values.get(key) {
            None => None,
            Some(value) => Some(&value),
        }
    }

    pub fn add(&mut self, key: &str, value: &str) {
        self.values
            .entry(key.to_owned())
            .and_modify(|e| e.push(value.to_owned()))
            .or_insert([value.to_owned()].to_vec());
    }

    pub fn add_many(&mut self, key: &str, value: &mut Vec<String>) {
        self.values
            .entry(key.to_owned())
            .and_modify(|e| e.append(value))
            .or_insert(value.clone());
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.values
            .insert(key.to_owned(), [value.to_owned()].to_vec());
    }

    pub fn set_many(&mut self, key: &str, value: &Vec<String>) {
        self.values.insert(key.to_owned(), value.clone());
    }

    pub fn from(search_string: &str) -> SearchParams {
        let search_split = search_string.split("&");
        let mut search = HashMap::<String, Vec<String>>::new();

        for param in search_split {
            if param.is_empty() {
                continue;
            };

            let (key, value) = match param.split_once("=") {
                Some((key, value)) => (key, value),
                None => (param, ""),
            };

            search
                .entry(key.to_owned())
                .and_modify(|e| e.push(value.to_owned()))
                .or_insert([value.to_owned()].to_vec());
        }

        SearchParams { values: search }
    }
}
