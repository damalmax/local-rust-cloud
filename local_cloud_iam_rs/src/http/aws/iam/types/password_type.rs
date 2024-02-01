use std::ops::Deref;

use local_cloud_validate::{ValidationError, ValidationErrorKind};

lazy_static::lazy_static! {
    static ref REGEX : regex::Regex =
    regex::Regex::new(r"^[\u0009\u000A\u000D\u0020-\u00FF]+$").unwrap();
}
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct PasswordType(String);

impl Deref for PasswordType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl local_cloud_validate::NamedValidator for &PasswordType {
    fn validate(&self, at: &str) -> Result<(), ValidationError> {
        local_cloud_validate::validate_str_length_min(Some(self), 1usize, at)
            .map_err(|err| ValidationError::new(ValidationErrorKind::Password, err.message))?;
        local_cloud_validate::validate_str_length_max(Some(self), 128usize, at)
            .map_err(|err| ValidationError::new(ValidationErrorKind::Password, err.message))?;
        local_cloud_validate::validate_regexp(Some(self), REGEX.deref(), at)
            .map_err(|err| ValidationError::new(ValidationErrorKind::Password, err.message))?;
        Ok(())
    }
}
