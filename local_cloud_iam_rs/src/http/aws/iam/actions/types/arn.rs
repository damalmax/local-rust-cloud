use std::ops::Deref;

use serde::Deserialize;

use local_cloud_validate::{
    validate_regexp, validate_str_length_max, validate_str_length_min, NamedValidator, ValidationError,
    ValidationErrorKind,
};

use crate::http::aws::iam::actions::types::utils::is_valid_input;
use crate::http::aws::iam::constants;

#[derive(Debug, Deserialize)]
pub(crate) struct ArnType(String);

impl Deref for ArnType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl NamedValidator for &ArnType {
    fn validate(&self, at: &str) -> Result<(), ValidationError> {
        validate_str_length_min(Some(&self), constants::arn::ARN_MIN_LENGTH, at)?;
        validate_str_length_max(Some(&self), constants::arn::ARN_MAX_LENGTH, at)?;
        Ok(())
    }
}
