use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FloatShape {
    pub traits: Option<FloatTraits>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FloatTraits {
    #[serde(rename = "smithy.api#default")]
    pub default_value: Option<f64>,
    #[serde(rename = "smithy.api#range")]
    pub range: Option<FloatTraitsRange>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FloatTraitsRange {
    pub min: Option<f64>,
    pub max: Option<f64>,
}
