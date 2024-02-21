use std::ops::Deref;

use local_cloud_validate::{validate_regexp, validate_str_length_max, validate_str_length_min};

lazy_static::lazy_static! {
    static ref REGEX : regex::Regex = regex::Regex::new(r"^[\p{L}\p{Z}\p{N}_.:/=+\-@]+$")
    .unwrap();
}
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct TagKeyType(String);

impl Deref for TagKeyType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl local_cloud_validate::NamedValidator for &TagKeyType {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        validate_str_length_min(Some(self), 1usize, at)?;
        validate_str_length_max(Some(self), 128usize, at)?;
        validate_regexp(Some(self), REGEX.deref(), at)?;
        Ok(())
    }
}
