use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{metadata::Metadata, shape::shape::Shape};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Smithy {
    pub smithy: String,
    pub metadata: Metadata,
    pub shapes: HashMap<String, Shape>,
}
