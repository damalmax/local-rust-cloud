use std::ops::Deref;
lazy_static::lazy_static! {
    static ref CHARACTERS : Vec < char > =
    "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+=,.@-_ ".chars()
    .into_iter().collect();
}
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct PolicyDescriptionType(String);
impl Deref for PolicyDescriptionType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl local_cloud_validate::NamedValidator for &PolicyDescriptionType {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_str_length_min(Some(&self), 0usize, at)?;
        local_cloud_validate::validate_str_length_max(Some(&self), 1000usize, at)?;
        local_cloud_validate::validate_chars(Some(&self), &CHARACTERS, at)?;
        Ok(())
    }
}
