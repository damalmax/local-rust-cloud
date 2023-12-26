use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntegerShape {
    pub traits: Option<IntegerTraits>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntegerTraitsRange {
    pub min: Option<i32>,
    pub max: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntegerTraits {
    #[serde(rename = "smithy.api#default")]
    pub default_value: Option<i32>,
    #[serde(rename = "smithy.api#range")]
    pub range: Option<IntegerTraitsRange>,
}
