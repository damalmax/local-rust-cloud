use std::ops::Deref;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ContextKeyNameType(String);

impl Deref for ContextKeyNameType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl local_cloud_validate::NamedValidator for &ContextKeyNameType {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_str_length_min(Some(&self), 5usize, at)?;
        local_cloud_validate::validate_str_length_max(Some(&self), 256usize, at)?;
        Ok(())
    }
}
