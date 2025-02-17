use std::ops::Deref;

lazy_static::lazy_static! {
    static ref REGEX : regex::Regex =
    regex::Regex::new(r"^[\u0009\u000A\u000D\u0020-\u007E\u00A1-\u00FF]*$").unwrap();
}
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct RoleDescriptionType(String);

impl Deref for RoleDescriptionType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl validators::NamedValidator for &RoleDescriptionType {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_str_length_min(Some(self), 0usize, at)?;
        validators::validate_str_length_max(Some(self), 1000usize, at)?;
        validators::validate_regexp(Some(self), REGEX.deref(), at)?;
        Ok(())
    }
}
