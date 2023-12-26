use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StructureShape {
    pub members: HashMap<String, StructureMember>,
    pub traits: Option<StructureTraits>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StructureMember {
    pub target: String,
    pub traits: Option<StructureMemberTraits>,
}

impl StructureMember {
    pub fn documentation(&self) -> Option<String> {
        self.traits.as_ref().and_then(|t| t.documentation.clone())
    }

    pub fn is_optional(&self) -> bool {
        if let Some(traits) = &self.traits {
            traits.required.is_none()
        } else {
            true
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StructureMemberTraits {
    #[serde(rename = "smithy.api#documentation")]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#xmlName")]
    pub xml_name: Option<String>,
    #[serde(rename = "aws.protocols#ec2QueryName")]
    pub ec2_query_name: Option<String>,
    #[serde(rename = "smithy.api#required")]
    pub required: Option<()>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StructureTraits {
    #[serde(rename = "smithy.api#documentation")]
    pub documentation: Option<String>,
    #[serde(rename = "smithy.api#error")]
    pub error: Option<String>,
    #[serde(rename = "smithy.api#httpError")]
    pub http_error: Option<i16>,
    #[serde(rename = "smithy.api#retryable")]
    pub retryable: Option<StructureTraitsRetryable>,
}

impl StructureShape {
    pub fn documentation(&self) -> Option<String> {
        if self.traits.is_none() {
            return Option::None;
        }
        let traits = self.traits.as_ref().unwrap();
        if let Some(docs) = &traits.documentation {
            return Option::Some(docs.to_string());
        }
        return Option::None;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StructureTraitsRetryable {
    pub throttling: Option<bool>,
}
