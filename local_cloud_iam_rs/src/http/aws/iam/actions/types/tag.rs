use serde::Deserialize;

use local_cloud_validate::{validate_named, validate_required, NamedValidator, ValidationError};

use crate::http::aws::iam::actions::types::tag_key::TagKeyType;
use crate::http::aws::iam::actions::types::tag_value::TagValueType;

#[derive(Debug, Deserialize)]
pub(crate) struct TagType {
    #[serde(rename = "Key")]
    pub key: Option<TagKeyType>,
    #[serde(rename = "Value")]
    pub value: Option<TagValueType>,
}

impl TagType {
    pub(crate) fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }

    pub(crate) fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
}

impl NamedValidator for &TagType {
    fn validate(&self, at: &str) -> Result<(), ValidationError> {
        let key_at = format!("{at}.key");
        validate_required(self.key(), &key_at)?;
        validate_named(self.key.as_ref(), &key_at)?;
        let value_at = format!("{at}.value");
        validate_named(self.value.as_ref(), &value_at)?;
        Ok(())
    }
}
