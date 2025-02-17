use std::ops::Deref;

lazy_static::lazy_static! {
    static ref REGEX : regex::Regex =
    regex::Regex::new(r"^v[1-9][0-9]*(\.[A-Za-z0-9-]*)?$").unwrap();
}
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct PolicyVersionIdType(String);

impl Deref for PolicyVersionIdType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl validators::NamedValidator for &PolicyVersionIdType {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_regexp(Some(self), REGEX.deref(), at)?;
        Ok(())
    }
}
