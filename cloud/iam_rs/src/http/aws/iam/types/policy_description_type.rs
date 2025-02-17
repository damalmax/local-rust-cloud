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

impl validators::NamedValidator for &PolicyDescriptionType {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_str_length_min(Some(self), 0usize, at)?;
        validators::validate_str_length_max(Some(self), 1000usize, at)?;
        validators::validate_chars(Some(self), &CHARACTERS, at)?;
        Ok(())
    }
}
