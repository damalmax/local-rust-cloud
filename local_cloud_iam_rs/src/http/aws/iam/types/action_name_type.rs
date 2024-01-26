use std::ops::Deref;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ActionNameType(String);

impl Deref for ActionNameType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl local_cloud_validate::NamedValidator for &ActionNameType {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_str_length_min(Some(&self), 3usize, at)?;
        local_cloud_validate::validate_str_length_max(Some(&self), 128usize, at)?;
        Ok(())
    }
}
