use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlobShape {
    pub traits: Option<BlobTraits>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlobTraits {
    #[serde(rename = "smithy.api#sensitive")]
    pub sensitive: Option<()>,
}
