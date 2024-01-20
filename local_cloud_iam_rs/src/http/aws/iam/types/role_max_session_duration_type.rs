use std::ops::Deref;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct RoleMaxSessionDurationType(i32);
impl Deref for RoleMaxSessionDurationType {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl local_cloud_validate::NamedValidator for &RoleMaxSessionDurationType {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_min(Some(self.0), 3600i32, at)?;
        local_cloud_validate::validate_max(Some(self.0), 43200i32, at)?;
        Ok(())
    }
}
