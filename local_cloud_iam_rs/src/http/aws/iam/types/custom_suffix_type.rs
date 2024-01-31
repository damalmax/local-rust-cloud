use std::ops::Deref;

lazy_static::lazy_static! {
    static ref REGEX : regex::Regex = regex::Regex::new(r"^[\w+=,.@-]+$").unwrap();
}
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct CustomSuffixType(String);

impl Deref for CustomSuffixType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl local_cloud_validate::NamedValidator for &CustomSuffixType {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_str_length_min(Some(self), 1usize, at)?;
        local_cloud_validate::validate_str_length_max(Some(self), 64usize, at)?;
        local_cloud_validate::validate_regexp(Some(self), REGEX.deref(), at)?;
        Ok(())
    }
}
