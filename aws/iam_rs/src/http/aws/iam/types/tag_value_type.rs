use std::ops::Deref;

lazy_static::lazy_static! {
    static ref REGEX : regex::Regex = regex::Regex::new(r"^[\p{L}\p{Z}\p{N}_.:/=+\-@]*$")
    .unwrap();
}
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct TagValueType(String);

impl Deref for TagValueType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl validators::NamedValidator for &TagValueType {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_str_length_min(Some(self), 0usize, at)?;
        validators::validate_str_length_max(Some(self), 256usize, at)?;
        validators::validate_regexp(Some(self), REGEX.deref(), at)?;
        Ok(())
    }
}
