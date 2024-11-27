use std::ops::Deref;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ResourceNameType(String);

impl Deref for ResourceNameType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl validators::NamedValidator for &ResourceNameType {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_str_length_min(Some(self), 1usize, at)?;
        validators::validate_str_length_max(Some(self), 2048usize, at)?;
        Ok(())
    }
}
