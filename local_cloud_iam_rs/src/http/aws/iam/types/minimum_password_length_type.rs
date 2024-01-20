use std::ops::Deref;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct MinimumPasswordLengthType(i32);
impl Deref for MinimumPasswordLengthType {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl local_cloud_validate::NamedValidator for &MinimumPasswordLengthType {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_min(Some(self.0), 6i32, at)?;
        local_cloud_validate::validate_max(Some(self.0), 128i32, at)?;
        Ok(())
    }
}
