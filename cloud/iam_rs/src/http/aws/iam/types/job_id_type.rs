use std::ops::Deref;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct JobIdType(String);

impl Deref for JobIdType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl validators::NamedValidator for &JobIdType {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_str_length_min(Some(self), 36usize, at)?;
        validators::validate_str_length_max(Some(self), 36usize, at)?;
        Ok(())
    }
}
