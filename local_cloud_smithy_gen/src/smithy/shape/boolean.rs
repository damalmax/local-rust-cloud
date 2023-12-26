use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BooleanShape {
    pub traits: Option<BooleanTraits>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BooleanTraits {
    #[serde(rename = "smithy.api#default")]
    pub default_value: bool,
}
