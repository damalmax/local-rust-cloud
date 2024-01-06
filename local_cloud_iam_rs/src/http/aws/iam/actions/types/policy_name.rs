use std::ops::Deref;

use serde::Deserialize;

use local_cloud_validate::{
    validate_str_length_max, validate_str_length_min, NamedValidator, ValidationError, ValidationErrorKind,
};

use crate::http::aws::iam::actions::types::utils::is_valid_input;
use crate::http::aws::iam::constants;

#[derive(Debug, Deserialize)]
pub(crate) struct PolicyNameType(String);

impl Deref for PolicyNameType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl NamedValidator for &PolicyNameType {
    fn validate(&self, at: &str) -> Result<(), ValidationError> {
        let value = self.trim();
        validate_str_length_min(Some(value), constants::policy::POLICY_NAME_MIN_LENGTH, at)?;
        validate_str_length_max(Some(value), constants::policy::POLICY_NAME_MAX_LENGTH, at)?;
        if !is_valid_input(value, &constants::policy::POLICY_NAME_VALID_CHARACTERS) {
            return Err(ValidationError::new(
                ValidationErrorKind::Allowed,
                format!("Please use alphanumeric, whitespaces and '+=,.@-_' characters for {at}."),
            ));
        }
        Ok(())
    }
}
