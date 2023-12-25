use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub suppressions: Option<Vec<Suppression>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Suppression {
    pub id: String,
    pub namespace: String,
}
