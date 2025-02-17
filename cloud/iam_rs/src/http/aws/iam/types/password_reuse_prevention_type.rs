use std::ops::Deref;

use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub(crate) struct PasswordReusePreventionType(i32);

impl Deref for PasswordReusePreventionType {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl validators::NamedValidator for &PasswordReusePreventionType {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_min(Some(self.0), 1i32, at)?;
        validators::validate_max(Some(self.0), 24i32, at)?;
        Ok(())
    }
}

impl<'de> Deserialize<'de> for PasswordReusePreventionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let buf: &str = Deserialize::deserialize(deserializer)?;
        buf.parse::<i32>()
            .map(PasswordReusePreventionType)
            .map_err(|_err| serde::de::Error::custom("Expected i32 but found string"))
    }
}
