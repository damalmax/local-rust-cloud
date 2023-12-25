use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OperationShape {
    pub input: OperationInput,
    pub output: OperationOutput,
    pub errors: Option<Vec<OperationError>>,
    pub traits: Option<OperationTraits>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OperationInput {
    pub target: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OperationOutput {
    pub target: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OperationError {
    pub target: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OperationTraits {
    #[serde(rename = "smithy.api#documentation")]
    pub documentation: Option<String>,
    pub examples: Option<Vec<OperationTraitsExample>>,
    #[serde(rename = "smithy.api#idempotent")]
    pub idempotent: Option<()>,
    #[serde(rename = "smithy.api#http")]
    pub http: Option<OperationTraitsHttp>,
    #[serde(rename = "smithy.api#readonly")]
    pub read_only: Option<()>,
    #[serde(rename = "smithy.api#suppress")]
    pub suppress: Option<Vec<OperationTraitsSuppress>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OperationTraitsSuppress {
    EventSource,
    HttpMethodSemantics,
    WaitableTraitInvalidErrorType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OperationTraitsHttp {
    pub uri: String,
    pub method: String,
    pub code: i16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OperationTraitsExample {
    pub title: Option<String>,
    pub documentation: Option<String>,
    pub input: HashMap<String, String>,
}