use serde::Deserialize;

use crate::http::aws::iam::actions::types::tag_key::TagKeyType;
use crate::http::aws::iam::actions::types::tag_value::TagValueType;

#[derive(Debug, Deserialize)]
pub(crate) struct LocalTag {
    #[serde(rename = "Key")]
    pub key: Option<TagKeyType>,
    #[serde(rename = "Value")]
    pub value: Option<TagValueType>,
}

impl LocalTag {
    pub(crate) fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }

    pub(crate) fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
}
