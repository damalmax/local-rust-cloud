use std::ops::Deref;

lazy_static::lazy_static! {
    static ref REGEX : regex::Regex =
    regex::Regex::new(r"^((/[A-Za-z0-9\.,\+@=_-]+)*)/$").unwrap();
}
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct PolicyPathType(String);

impl Deref for PolicyPathType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl validators::NamedValidator for &PolicyPathType {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_str_length_min(Some(self), 1usize, at)?;
        validators::validate_str_length_max(Some(self), 512usize, at)?;
        validators::validate_regexp(Some(self), REGEX.deref(), at)?;
        Ok(())
    }
}
