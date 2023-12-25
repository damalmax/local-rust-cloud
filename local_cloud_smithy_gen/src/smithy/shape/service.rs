use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceShape {
    pub version: Option<String>,
    pub operations: Vec<ServiceOperation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceOperation {
    pub target: String,
}
