use std::ops::Deref;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct JobIdType(String);

impl Deref for JobIdType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl local_cloud_validate::NamedValidator for &JobIdType {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_str_length_min(Some(self), 36usize, at)?;
        local_cloud_validate::validate_str_length_max(Some(self), 36usize, at)?;
        Ok(())
    }
}
