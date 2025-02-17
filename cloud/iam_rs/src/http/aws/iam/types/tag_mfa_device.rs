use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct TagMfaDeviceRequest {
    #[serde(rename = "Tags")]
    pub(crate) tags: Option<Vec<types::tag::Tag>>,
    #[serde(rename = "SerialNumber")]
    pub(crate) serial_number: Option<types::serial_number_type::SerialNumberType>,
}

impl TagMfaDeviceRequest {
    pub(crate) fn tags(&self) -> Option<&[types::tag::Tag]> {
        self.tags.as_deref()
    }
    pub(crate) fn serial_number(&self) -> Option<&str> {
        self.serial_number.as_deref()
    }
}

impl validators::NamedValidator for &TagMfaDeviceRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.tags(), format!("{at}.{}", "Tags").as_str())?;
        validators::validate_array_size_min(self.tags(), 0usize, format!("{at}.{}", "Tags").as_str())?;
        validators::validate_array_size_max(self.tags(), 50usize, format!("{at}.{}", "Tags").as_str())?;
        if let Some(tags) = self.tags() {
            for (id, member) in tags.iter().enumerate() {
                validators::validate_named(Some(member), format!("{at}.{}.member.{id}", "Tags").as_str())?;
            }
        }
        validators::validate_required(self.serial_number(), format!("{at}.{}", "SerialNumber").as_str())?;
        validators::validate_named(self.serial_number.as_ref(), format!("{at}.{}", "SerialNumber").as_str())?;
        Ok(())
    }
}
