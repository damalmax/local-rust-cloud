use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnumShape {
    pub members: HashMap<String, EnumMember>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnumMember {
    pub target: String,
    pub traits: EnumTraits,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnumTraits {
    #[serde(rename = "smithy.api#enumValue")]
    pub enum_value: String,
}
