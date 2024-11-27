use std::ops::Deref;

lazy_static::lazy_static! {
    static ref REGEX : regex::Regex =
    regex::Regex::new(r"^o-[0-9a-z]{10,32}\/r-[0-9a-z]{4,32}[0-9a-z-\/]*$").unwrap();
}
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct OrganizationsEntityPathType(String);

impl Deref for OrganizationsEntityPathType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl validators::NamedValidator for &OrganizationsEntityPathType {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_str_length_min(Some(self), 19usize, at)?;
        validators::validate_str_length_max(Some(self), 427usize, at)?;
        validators::validate_regexp(Some(self), REGEX.deref(), at)?;
        Ok(())
    }
}
