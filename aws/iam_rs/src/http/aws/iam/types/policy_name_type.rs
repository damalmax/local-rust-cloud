use std::ops::Deref;

lazy_static::lazy_static! {
    static ref REGEX : regex::Regex = regex::Regex::new(r"^[\w+=,.@-]+$").unwrap();
    static ref CHARACTERS : Vec < char > =
    "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+=,.@-_".chars()
    .into_iter().collect();
}
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct PolicyNameType(String);

impl Deref for PolicyNameType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl validators::NamedValidator for &PolicyNameType {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_str_length_min(Some(self), 1usize, at)?;
        validators::validate_str_length_max(Some(self), 128usize, at)?;
        validators::validate_chars(Some(self), &CHARACTERS, at)?;
        validators::validate_regexp(Some(self), REGEX.deref(), at)?;
        Ok(())
    }
}
