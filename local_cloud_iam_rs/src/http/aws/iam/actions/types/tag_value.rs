use std::ops::Deref;

use serde::Deserialize;

use local_cloud_validate::{validate_regexp, validate_str_length_max, NamedValidator, ValidationError};

use crate::http::aws::iam::constants;

#[derive(Debug, Deserialize)]
pub(crate) struct TagValueType(String);

impl Deref for TagValueType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl NamedValidator for &TagValueType {
    fn validate(&self, at: &str) -> Result<(), ValidationError> {
        validate_str_length_max(Some(&self), constants::tag::TAG_VALUE_MAX_LENGTH, at)?;
        validate_regexp(Some(&self), constants::tag::TAG_VALUE_REGEX.deref(), at)?;
        Ok(())
    }
}
