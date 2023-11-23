use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnionShape {
    pub members: HashMap<String, UnionMember>,
    pub traits: Option<UnionTraits>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnionTraits {
    #[serde(rename = "smithy.api#documentation")]
    pub documentation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnionMember {
    pub target: String,
    pub traits: Option<UnionMemberTraits>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnionMemberTraits {
    #[serde(rename = "smithy.api#documentation")]
    pub documentation: Option<String>,
}
