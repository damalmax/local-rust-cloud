use std::ops::Deref;

lazy_static::lazy_static! {
    static ref REGEX : regex::Regex = regex::Regex::new(r"^[\w]+$").unwrap();
}
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ServiceSpecificCredentialId(String);

impl Deref for ServiceSpecificCredentialId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl validators::NamedValidator for &ServiceSpecificCredentialId {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_str_length_min(Some(self), 20usize, at)?;
        validators::validate_str_length_max(Some(self), 128usize, at)?;
        validators::validate_regexp(Some(self), REGEX.deref(), at)?;
        Ok(())
    }
}
