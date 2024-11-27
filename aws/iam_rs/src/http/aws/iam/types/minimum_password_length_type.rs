use std::ops::Deref;

use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub(crate) struct MinimumPasswordLengthType(i32);

impl Deref for MinimumPasswordLengthType {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl validators::NamedValidator for &MinimumPasswordLengthType {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_min(Some(self.0), 6i32, at)?;
        validators::validate_max(Some(self.0), 128i32, at)?;
        Ok(())
    }
}

impl<'de> Deserialize<'de> for MinimumPasswordLengthType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let buf: &str = Deserialize::deserialize(deserializer)?;
        buf.parse::<i32>()
            .map(MinimumPasswordLengthType)
            .map_err(|_err| serde::de::Error::custom("Expected i32 but found string"))
    }
}
