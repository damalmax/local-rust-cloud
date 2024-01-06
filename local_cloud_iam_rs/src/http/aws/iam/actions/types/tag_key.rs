use std::ops::Deref;

use serde::Deserialize;

use local_cloud_validate::{
    validate_regexp, validate_str_length_max, validate_str_length_min, NamedValidator, ValidationError,
};

use crate::http::aws::iam::constants;

#[derive(Debug, Deserialize)]
pub(crate) struct TagKeyType(String);

impl Deref for TagKeyType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl NamedValidator for &TagKeyType {
    fn validate(&self, at: &str) -> Result<(), ValidationError> {
        validate_str_length_min(Some(&self), constants::tag::TAG_KEY_MIN_LENGTH, at)?;
        validate_str_length_max(Some(&self), constants::tag::TAG_KEY_MAX_LENGTH, at)?;
        validate_regexp(Some(&self), constants::tag::TAG_KEY_REGEX.deref(), at)?;
        Ok(())
    }
}
