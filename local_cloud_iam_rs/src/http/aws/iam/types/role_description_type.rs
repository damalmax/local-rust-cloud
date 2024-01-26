use std::ops::Deref;

lazy_static::lazy_static! {
    static ref REGEX : regex::Regex =
    regex::Regex::new(r"^[\u0009\u000A\u000D\u0020-\u007E\u00A1-\u00FF]*$").unwrap();
}
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct RoleDescriptionType(String);

impl Deref for RoleDescriptionType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl local_cloud_validate::NamedValidator for &RoleDescriptionType {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_str_length_min(Some(&self), 0usize, at)?;
        local_cloud_validate::validate_str_length_max(Some(&self), 1000usize, at)?;
        local_cloud_validate::validate_regexp(Some(&self), REGEX.deref(), at)?;
        Ok(())
    }
}
