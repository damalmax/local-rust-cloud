use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryReader {
    params: HashMap<String, String>,
}

impl QueryReader {
    pub fn new(params: HashMap<String, String>) -> Self {
        QueryReader { params }
    }

    pub fn get_string(&self, key: impl Into<String>) -> Option<String> {
        let key = key.into();
        if self.params.contains_key(key.as_str()) {
            Option::Some(self.params.get(key.as_str()).unwrap().to_string())
        } else {
            Option::None
        }
    }

    pub fn get_i32(&self, key: impl Into<String>) -> Option<i32> {
        self.get_string(key).map(|v| v.parse::<i32>().expect("Failed to parse property"))
    }

    pub fn get_i32_or_default(&self, key: impl Into<String>, default: i32) -> Option<i32> {
        self.get_i32(key).or_else(|| Option::Some(default))
    }
}
