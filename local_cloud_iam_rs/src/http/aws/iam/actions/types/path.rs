use std::ops::Deref;

use serde::Deserialize;

use local_cloud_validate::{
    validate_regexp, validate_str_length_max, validate_str_length_min, NamedValidator, ValidationError,
};

use crate::http::aws::iam::constants;

#[derive(Debug, Deserialize)]
pub(crate) struct PathType(String);

impl Deref for PathType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl NamedValidator for &PathType {
    fn validate(&self, at: &str) -> Result<(), ValidationError> {
        validate_str_length_min(Some(&self), constants::policy::PATH_MIN_LENGTH, at)?;
        validate_str_length_max(Some(&self), constants::policy::PATH_MAX_LENGTH, at)?;
        validate_regexp(Some(&self), constants::policy::POLICY_PATH_REGEX.deref(), at)?;
        Ok(())
    }
}
