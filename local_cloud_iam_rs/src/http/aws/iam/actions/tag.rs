use serde::Deserialize;

use crate::http::aws::iam::actions::validate::{IamValidator, ValidationError};
use crate::http::aws::iam::constants;

#[derive(Debug, Deserialize)]
pub(crate) struct LocalTag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

impl LocalTag {
    pub(crate) fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }

    pub(crate) fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    pub(crate) fn validate(&self, tag_index: usize) -> Result<(), ValidationError> {
        if self.key().is_none() {
            return Err(ValidationError::TagNoKey {
                at: format!("Tags.member.{tag_index}.key"),
            });
        }
        if self.value().is_none() {
            return Err(ValidationError::TagNoValue {
                at: format!("Tags.member.{tag_index}.value"),
            });
        }
        let key = self.key().unwrap();
        let key_length = key.chars().count();
        if key_length < constants::tag::TAG_KEY_MIN_SIZE {
            return Err(ValidationError::TagKeyMinLength {
                size: key_length,
                min: constants::tag::TAG_KEY_MIN_SIZE,
                at: format!("Tags.member.{tag_index}.key"),
            });
        }
        if key_length > constants::tag::TAG_KEY_MAX_SIZE {
            return Err(ValidationError::TagKeyMaxLength {
                size: key_length,
                max: constants::tag::TAG_KEY_MAX_SIZE,
                at: format!("Tags.member.{tag_index}.key"),
            });
        }
        let value = self.value().unwrap();
        let value_length = value.chars().count();
        if value_length < constants::tag::TAG_VALUE_MIN_SIZE {
            return Err(ValidationError::TagValueMinLength {
                size: value_length,
                min: constants::tag::TAG_VALUE_MIN_SIZE,
                at: format!("Tags.member.{tag_index}.value"),
            });
        }
        if value_length > constants::tag::TAG_VALUE_MAX_SIZE {
            return Err(ValidationError::TagValueMaxLength {
                size: value_length,
                max: constants::tag::TAG_VALUE_MAX_SIZE,
                at: format!("Tags.member.{tag_index}.value"),
            });
        }
        return Ok(());
    }
}
