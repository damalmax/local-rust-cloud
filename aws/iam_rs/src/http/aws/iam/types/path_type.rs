use std::ops::Deref;

use validators::{
    validate_regexp, validate_str_length_max, validate_str_length_min, ValidationError, ValidationErrorKind,
};

lazy_static::lazy_static! {
    static ref REGEX : regex::Regex =
    regex::Regex::new(r"^(\u002F)|(\u002F[\u0021-\u007E]+\u002F)$").unwrap();
}
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct PathType(String);

impl Deref for PathType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl validators::NamedValidator for &PathType {
    fn validate(&self, at: &str) -> Result<(), ValidationError> {
        validate_str_length_min(Some(self), 1usize, at)?;
        validate_str_length_max(Some(self), 512usize, at)?;
        validate_regexp(Some(self), REGEX.deref(), at)?;

        if !self.0.ends_with('/') {
            return Err(ValidationError::new(
                ValidationErrorKind::Other,
                format!("'{at}' must begin with slash ('/') and must include a trailing slash."),
            ));
        }

        Ok(())
    }
}
