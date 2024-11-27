use validators::{validate_array_size_max, validate_array_size_min, validate_named, validate_required};

use crate::http::aws::iam::types;
use crate::http::aws::iam::types::tag_key_type::TagKeyType;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UntagMfaDeviceRequest {
    #[serde(rename = "TagKeys")]
    pub(crate) tag_keys: Option<Vec<TagKeyType>>,
    #[serde(rename = "SerialNumber")]
    pub(crate) serial_number: Option<types::serial_number_type::SerialNumberType>,
}

impl UntagMfaDeviceRequest {
    pub(crate) fn tag_keys(&self) -> Vec<String> {
        match &self.tag_keys {
            None => vec![],
            Some(keys) => keys.iter().map(|k| k.to_string()).collect::<Vec<_>>(),
        }
    }
    pub(crate) fn tag_keys_type(&self) -> Option<&[TagKeyType]> {
        self.tag_keys.as_deref()
    }
    pub(crate) fn serial_number(&self) -> Option<&str> {
        self.serial_number.as_deref()
    }
}

impl validators::NamedValidator for &UntagMfaDeviceRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validate_required(self.tag_keys_type(), format!("{at}.{}", "TagKeys").as_str())?;
        validate_array_size_min(self.tag_keys_type(), 0usize, format!("{at}.{}", "TagKeys").as_str())?;
        validate_array_size_max(self.tag_keys_type(), 50usize, format!("{at}.{}", "TagKeys").as_str())?;
        if let Some(tag_keys) = self.tag_keys_type() {
            for (id, member) in tag_keys.iter().enumerate() {
                validate_named(Some(member), format!("{at}.{}.member.{id}", "TagKeys").as_str())?;
            }
        }
        validate_required(self.serial_number(), format!("{at}.{}", "SerialNumber").as_str())?;
        validate_named(self.serial_number.as_ref(), format!("{at}.{}", "SerialNumber").as_str())?;
        Ok(())
    }
}
