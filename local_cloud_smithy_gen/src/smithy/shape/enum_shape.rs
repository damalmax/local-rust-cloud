use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnumShape {
    pub members: HashMap<String, EnumMember>,
    pub traits: Option<EnumTraits>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnumTraits {
    #[serde(rename = "smithy.api#documentation")]
    pub documentation: Option<String>,
}

impl EnumShape {
    pub fn documentation(&self) -> Option<String> {
        self.traits.as_ref().and_then(|t| t.documentation.clone())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnumMember {
    pub target: String,
    pub traits: EnumMemberTraits,
}

impl EnumMember {
    pub fn documentation(&self) -> Option<String> {
        self.traits.documentation.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnumMemberTraits {
    #[serde(rename = "smithy.api#enumValue")]
    pub enum_value: String,
    #[serde(rename = "smithy.api#documentation")]
    pub documentation: Option<String>,
}
