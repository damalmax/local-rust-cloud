use std::ops::Deref;

use serde::de::Error;
use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub struct Bool(bool);

impl Bool {
    pub fn as_bool(&self) -> bool {
        self.0
    }
}

impl Deref for Bool {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Bool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        match s {
            "true" => Ok(Bool(true)),
            "false" => Ok(Bool(false)),
            _ => Err(Error::unknown_variant(s, &["true", "false"])),
        }
    }
}
