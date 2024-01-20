use std::ops::Deref;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct MaxPasswordAgeType(i32);
impl Deref for MaxPasswordAgeType {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl local_cloud_validate::NamedValidator for &MaxPasswordAgeType {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_min(Some(self.0), 1i32, at)?;
        local_cloud_validate::validate_max(Some(self.0), 1095i32, at)?;
        Ok(())
    }
}
