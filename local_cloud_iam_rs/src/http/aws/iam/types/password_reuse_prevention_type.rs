use std::ops::Deref;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct PasswordReusePreventionType(i32);
impl Deref for PasswordReusePreventionType {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl local_cloud_validate::NamedValidator for &PasswordReusePreventionType {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_min(Some(self.0), 1i32, at)?;
        local_cloud_validate::validate_max(Some(self.0), 24i32, at)?;
        Ok(())
    }
}
