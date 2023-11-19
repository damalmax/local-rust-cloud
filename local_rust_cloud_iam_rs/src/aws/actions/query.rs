use std::{collections::HashMap, ops::Deref};

use local_rust_cloud_common::request::QueryReader as CommonQueryReader;

#[derive(Debug)]
pub struct QueryReader(CommonQueryReader);

impl QueryReader {
    pub fn new(params: HashMap<String, String>) -> Self {
        QueryReader(CommonQueryReader::new(params))
    }
}

impl Deref for QueryReader {
    type Target = CommonQueryReader;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
