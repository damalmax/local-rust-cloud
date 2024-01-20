use std::ops::Deref;
lazy_static::lazy_static! {
    static ref REGEX : regex::Regex = regex::Regex::new(r"^p-[0-9a-zA-Z_]{8,128}$")
    .unwrap();
}
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct OrganizationsPolicyIdType(String);
impl Deref for OrganizationsPolicyIdType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl local_cloud_validate::NamedValidator for &OrganizationsPolicyIdType {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_regexp(Some(&self), REGEX.deref(), at)?;
        Ok(())
    }
}
