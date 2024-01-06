use std::ops::Deref;

use serde::Deserialize;

use local_cloud_validate::{
    validate_regexp, validate_str_length_max, validate_str_length_min, NamedValidator, ValidationError,
};

use crate::http::aws::iam::constants;

#[derive(Debug, Deserialize)]
pub(crate) struct MarkerType(String);

impl Deref for MarkerType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl NamedValidator for MarkerType {
    fn validate(&self, field: &str) -> Result<(), ValidationError> {
        validate_str_length_min(Some(&self), constants::marker::MARKER_MIN_SIZE, field)?;
        validate_str_length_max(Some(&self), constants::marker::MARKER_MAX_SIZE, field)?;
        validate_regexp(Some(&self), constants::marker::MARKER_REGEX.deref(), field)?;
        Ok(())
    }
}
