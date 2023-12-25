use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LongShape {
    pub traits: Option<LongTraits>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LongTraits {
    #[serde(rename = "smithy.api#default")]
    pub default_value: Option<i64>,
    #[serde(rename = "smithy.api#range")]
    pub range: Option<LongTraitsRange>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LongTraitsRange {
    pub min: Option<i64>,
    pub max: Option<i64>,
}
