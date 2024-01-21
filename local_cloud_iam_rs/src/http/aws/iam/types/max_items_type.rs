use std::ops::Deref;

use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub(crate) struct MaxItemsType(i32);
impl Deref for MaxItemsType {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl local_cloud_validate::NamedValidator for &MaxItemsType {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_min(Some(self.0), 1i32, at)?;
        local_cloud_validate::validate_max(Some(self.0), 1000i32, at)?;
        Ok(())
    }
}

impl<'de> Deserialize<'de> for MaxItemsType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let buf: &str = Deserialize::deserialize(deserializer)?;
        buf.parse::<i32>()
            .map(|v| MaxItemsType(v))
            .map_err(|_err| serde::de::Error::custom("Expected i32 but found string"))
    }
}
