use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DoubleShape {
    pub traits: Option<DoubleTraits>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DoubleTraitsRange {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DoubleTraits {
    #[serde(rename = "smithy.api#default")]
    pub default_value: Option<f64>,
    #[serde(rename = "smithy.api#range")]
    pub range: Option<DoubleTraitsRange>,
}
