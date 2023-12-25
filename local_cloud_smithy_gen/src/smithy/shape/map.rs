use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MapShape {
    pub key: MapKey,
    pub value: MapValue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MapKey {
    pub target: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MapValue {
    pub target: String,
}
