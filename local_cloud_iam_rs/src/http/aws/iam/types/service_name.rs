use std::ops::Deref;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ServiceName(String);
impl Deref for ServiceName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl local_cloud_validate::NamedValidator for &ServiceName {
    fn validate(&self, _at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        Ok(())
    }
}
