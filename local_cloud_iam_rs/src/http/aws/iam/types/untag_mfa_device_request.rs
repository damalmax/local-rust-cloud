use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UntagMfaDeviceRequest {
    #[serde(rename = "TagKeys")]
    pub(crate) tag_keys: Option<Vec<types::tag_key_type::TagKeyType>>,
    #[serde(rename = "SerialNumber")]
    pub(crate) serial_number: Option<types::serial_number_type::SerialNumberType>,
}
impl UntagMfaDeviceRequest {
    pub(crate) fn tag_keys(&self) -> Option<&[types::tag_key_type::TagKeyType]> {
        self.tag_keys.as_deref()
    }
    pub(crate) fn serial_number(&self) -> Option<&str> {
        self.serial_number.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &UntagMfaDeviceRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(
            self.tag_keys(),
            format!("{at}.{}", "TagKeys").as_str(),
        )?;
        local_cloud_validate::validate_array_size_min(
            self.tag_keys(),
            0usize,
            format!("{at}.{}", "TagKeys").as_str(),
        )?;
        local_cloud_validate::validate_array_size_max(
            self.tag_keys(),
            50usize,
            format!("{at}.{}", "TagKeys").as_str(),
        )?;
        if let Some(tag_keys) = self.tag_keys() {
            for (id, member) in tag_keys.iter().enumerate() {
                local_cloud_validate::validate_named(
                    Some(member),
                    format!("{at}.{}.member.{id}", "TagKeys").as_str(),
                )?;
            }
        }
        local_cloud_validate::validate_required(
            self.serial_number(),
            format!("{at}.{}", "SerialNumber").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.serial_number.as_ref(),
            format!("{at}.{}", "SerialNumber").as_str(),
        )?;
        Ok(())
    }
}
