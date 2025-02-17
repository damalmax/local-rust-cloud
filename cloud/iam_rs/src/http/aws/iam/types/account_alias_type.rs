use std::ops::Deref;

lazy_static::lazy_static! {
    static ref REGEX : regex::Regex =
    regex::Regex::new(r"^[a-z0-9]([a-z0-9]|-(?!-)){1,61}[a-z0-9]$").unwrap();
}
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct AccountAliasType(String);

impl Deref for AccountAliasType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl validators::NamedValidator for &AccountAliasType {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_str_length_min(Some(self), 3usize, at)?;
        validators::validate_str_length_max(Some(self), 63usize, at)?;
        validators::validate_regexp(Some(self), REGEX.deref(), at)?;
        Ok(())
    }
}
