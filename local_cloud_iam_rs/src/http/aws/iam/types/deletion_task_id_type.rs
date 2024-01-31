use std::ops::Deref;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct DeletionTaskIdType(String);

impl Deref for DeletionTaskIdType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl local_cloud_validate::NamedValidator for &DeletionTaskIdType {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_str_length_min(Some(self), 1usize, at)?;
        local_cloud_validate::validate_str_length_max(Some(self), 1000usize, at)?;
        Ok(())
    }
}
