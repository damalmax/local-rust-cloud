use std::ops::Deref;
lazy_static::lazy_static! {
    static ref REGEX : regex::Regex = regex::Regex::new(r"^[\d]+$").unwrap();
}
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct AuthenticationCodeType(String);
impl Deref for AuthenticationCodeType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl local_cloud_validate::NamedValidator for &AuthenticationCodeType {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_str_length_min(Some(&self), 6usize, at)?;
        local_cloud_validate::validate_str_length_max(Some(&self), 6usize, at)?;
        local_cloud_validate::validate_regexp(Some(&self), REGEX.deref(), at)?;
        Ok(())
    }
}
