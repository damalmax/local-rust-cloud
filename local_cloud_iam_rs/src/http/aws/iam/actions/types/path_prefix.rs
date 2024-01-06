use std::ops::Deref;

use serde::Deserialize;

use local_cloud_validate::{
    validate_regexp, validate_str_length_max, validate_str_length_min, NamedValidator, ValidationError,
    ValidationErrorKind,
};

use crate::http::aws::iam::actions::types::utils::is_valid_input;
use crate::http::aws::iam::constants;

#[derive(Debug, Deserialize)]
pub(crate) struct PathPrefixType(String);

impl Deref for PathPrefixType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl NamedValidator for &PathPrefixType {
    fn validate(&self, at: &str) -> Result<(), ValidationError> {
        validate_str_length_min(Some(&self), constants::policy::PATH_MIN_LENGTH, at)?;
        validate_str_length_max(Some(&self), constants::policy::PATH_MAX_LENGTH, at)?;
        validate_regexp(Some(&self), constants::policy::POLICY_PATH_PREFIX_REGEX.deref(), at)?;
        Ok(())
    }
}
