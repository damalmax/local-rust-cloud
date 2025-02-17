use std::ops::Deref;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ContextKeyNameType(String);

impl Deref for ContextKeyNameType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl validators::NamedValidator for &ContextKeyNameType {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_str_length_min(Some(self), 5usize, at)?;
        validators::validate_str_length_max(Some(self), 256usize, at)?;
        Ok(())
    }
}
