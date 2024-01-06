use std::ops::Deref;

use serde::Deserialize;

use local_cloud_validate::{validate_str_length_max, NamedValidator, ValidationError, ValidationErrorKind};

use crate::http::aws::iam::actions::types::utils::is_valid_input;
use crate::http::aws::iam::constants;

#[derive(Debug, Deserialize)]
pub(crate) struct PolicyDescriptionType(String);

impl Deref for PolicyDescriptionType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl NamedValidator for &PolicyDescriptionType {
    fn validate(&self, at: &str) -> Result<(), ValidationError> {
        validate_str_length_max(Some(&self), constants::policy::POLICY_DESCRIPTION_MAX_LENGTH, at)?;
        if !is_valid_input(&self, &constants::policy::POLICY_DESCRIPTION_VALID_CHARACTERS) {
            return Err(ValidationError::new(
                ValidationErrorKind::Allowed,
                format!("Please use alphanumeric, whitespaces and '+=,.@-_' characters for {at}."),
            ));
        }
        Ok(())
    }
}
