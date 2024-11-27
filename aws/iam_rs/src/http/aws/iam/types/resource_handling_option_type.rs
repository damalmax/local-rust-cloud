use std::ops::Deref;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ResourceHandlingOptionType(String);

impl Deref for ResourceHandlingOptionType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl validators::NamedValidator for &ResourceHandlingOptionType {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_str_length_min(Some(self), 1usize, at)?;
        validators::validate_str_length_max(Some(self), 64usize, at)?;
        Ok(())
    }
}
